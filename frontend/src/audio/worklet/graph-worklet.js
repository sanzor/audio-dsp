/**
 * GraphWorklet — AudioWorkletProcessor
 *
 * Runs on the dedicated audio thread.
 *
 * All incoming messages are routed through WorkletMessageDispatcher which
 * maps each message type to a handler method on GraphWorklet.
 *
 * Messages in (main → worklet):
 *   SET_GRAPH     — new compiled graph + WASM binaries
 *   SET_BYPASS    — toggle raw pass-through
 *   UPDATE_PARAMS — update one node's params without recompiling
 *
 * Messages out (worklet → main):
 *   GRAPH_READY   — new graph is instantiated and active
 *   MODULE_ERROR  — one WASM module failed to instantiate
 *   RUNTIME_ERROR — a WASM module trapped inside process(); the graph is
 *                   retired (dry pass-through) until the next SET_GRAPH
 *   CPU_LOAD      — throttled processing-time-vs-quantum-budget ratio
 *
 * On SET_GRAPH the worklet:
 *   1. Compiles each node's WASM module once, then instantiates it once per
 *      channel (see "Channel layout" below)
 *   2. Generates a dedicated transform function from the graph topology
 *      (no interpretation loop at runtime — flat sequential calls)
 *   3. Pre-allocates every buffer the graph needs, then atomically swaps in
 *      the new graph
 *
 * Channel layout: the node is created with GRAPH_WORKLET_NODE_OPTIONS
 * (graphWorkletNodeOptions.ts) — exactly CHANNEL_COUNT (2) channels in and
 * out, `speakers` interpretation, so a mono source arrives upmixed to L=R and
 * anything wider is downmixed by the browser before reaching process(). Each
 * channel runs through its own "lane": its own WASM instances (independent
 * linear memory, so a stateful transform such as a filter or delay keeps
 * per-channel state instead of interleaving L/R through one state), its own
 * buffers and its own feedback history. Params are shared across lanes.
 * Processing is therefore dual-mono / unlinked: a sidechain or detector port
 * sees only its own channel's signal.
 *
 * Real-time safety: process() allocates nothing. Every per-quantum buffer
 * (port mixes, node outputs, the graph output, feedback history) is created
 * once in _loadGraph, and callWasm copies through a cached Float32Array view
 * of each instance's memory instead of creating views per call.
 *
 * Fault isolation: transform-sdk builds with panic = "abort", so a panicking
 * transform traps. A trapped instance is unrecoverable (its state Mutex is
 * never unlocked), so on any exception the worklet retires the whole graph,
 * passes dry audio through, and posts RUNTIME_ERROR once — instead of letting
 * the exception escape process(), which would kill the processor for good.
 *
 * NOTE: new Function() is used for code generation. Ensure your CSP allows
 * 'unsafe-eval' in the worker-src / script-src directives.
 */

// ─── Constants ────────────────────────────────────────────────────────────────

export const BLOCK_SIZE = 128;
// Must match GRAPH_WORKLET_NODE_OPTIONS.outputChannelCount / channelCount.
export const CHANNEL_COUNT = 2;
// Report CPU load every N quanta rather than every quantum (~370/sec at 48kHz)
// to avoid flooding the message port.
const CPU_REPORT_INTERVAL_QUANTA = 20;

// ─── WorkletMessageDispatcher ─────────────────────────────────────────────────
// Routes raw port messages to typed handler methods.

class WorkletMessageDispatcher {
  constructor(port, worklet) {
    port.onmessage = ({ data }) => {
      const handler = this.handlers[data.type];
      if (handler) handler(data);
      else console.warn('[graph-worklet] Unknown message type:', data.type);
    };

    this.handlers = {
      SET_GRAPH:     (msg) => worklet.onSetGraph(msg.graph, msg.binaries),
      SET_BYPASS:    (msg) => worklet.onSetBypass(msg.bypass),
      UPDATE_PARAMS: (msg) => worklet.onUpdateParams(msg.nodeIndex, msg.params),
    };
  }
}

// ─── GraphFunctionGenerator ────────────────────────────────────────────────────
// Generates a dedicated transform function from a CompiledGraph.
// Called once per SET_GRAPH — not at runtime per quantum.
//
// The generated function has the signature
//   (rawAudio, lane, params, callWasm, addInto) => Float32Array
// and writes only into buffers the lane already owns (see createLane), so it
// allocates nothing per quantum. It records the node it is about to run in
// `lane.current` so a trap can be attributed to a node.

export function generateTransformFunction(compiledGraph) {
  const { executionOrder } = compiledGraph;
  const lines = [];

  lines.push('const { instances, buffers, prev, ports, portLists, outs, output } = lane;');
  lines.push('output.fill(0);');

  for (let i = 0; i < executionOrder.length; i++) {
    const node = executionOrder[i];

    // One buffer per declared input port: each port's own sources (if any)
    // are summed together, independently of every other port's buffer — a
    // node with a Program input and a Sidechain input must not have those
    // two signals mixed into one before reaching its own process() call.
    // An unwired port's buffer is simply left zeroed (silence).
    node.inputs.forEach((bucket, j) => {
      const portBuf = `ports[${i}][${j}]`;
      bucket.forEach((src, k) => {
        let srcExpr;
        if (src.kind === 'raw') srcExpr = 'rawAudio';
        else if (src.kind === 'feedback') srcExpr = `prev[${src.bufferIndex}]`;
        else srcExpr = `buffers[${src.bufferIndex}]`;

        lines.push(k === 0 ? `${portBuf}.set(${srcExpr});` : `addInto(${portBuf}, ${srcExpr});`);
      });
    });

    lines.push(`lane.current = ${i};`);
    lines.push(`const out_${i} = outs[${i}];`);
    lines.push(`callWasm(instances[${i}], params[${i}], portLists[${i}], out_${i});`);

    // These two writes are independent, not mutually exclusive: a node can
    // hold a real buffer slot (outputBufferIndex !== -1, e.g. it has a
    // back-edge out) AND also have no forward consumer, in which case its
    // output must be written into the buffer for feedback AND summed into
    // the worklet's audible output.
    if (node.outputBufferIndex !== -1) {
      lines.push(`buffers[${node.outputBufferIndex}].set(out_${i});`);
    }
    if (node.writesToOutput) {
      lines.push(`addInto(output, out_${i});`);
    }
  }

  // Snapshot loop-edge buffers for the next quantum. Every read of prev[]
  // above has already happened, so overwriting in place is safe.
  const loopEdgeIndices = compiledGraph.feedbackBufferIndices ?? [];
  for (const idx of loopEdgeIndices) {
    lines.push(`prev[${idx}].set(buffers[${idx}]);`);
  }

  lines.push('lane.current = -1;');
  lines.push('return output;');

  return new Function(
    'rawAudio', 'lane', 'params', 'callWasm', 'addInto',
    lines.join('\n'),
  );
}

// ─── Runtime helpers (passed into the generated function) ─────────────────────

export function addInto(target, source) {
  for (let i = 0; i < BLOCK_SIZE; i++) target[i] += source[i];
}

// Pre-allocates everything one channel needs to run `compiledGraph`, so the
// generated function never allocates. `instances` are this lane's own WASM
// instances (null for a node whose module failed to load).
export function createLane(compiledGraph, instances) {
  const { executionOrder, bufferCount } = compiledGraph;
  const ports = executionOrder.map((node) =>
    node.inputs.map(() => new Float32Array(BLOCK_SIZE)),
  );
  return {
    instances,
    buffers:   Array.from({ length: bufferCount }, () => new Float32Array(BLOCK_SIZE)),
    prev:      Array.from({ length: bufferCount }, () => new Float32Array(BLOCK_SIZE)),
    ports,
    // Same Float32Array objects as `ports`, kept as the stable array callWasm
    // receives — avoids building `[port_0, port_1, ...]` every quantum.
    portLists: ports,
    outs:      executionOrder.map(() => new Float32Array(BLOCK_SIZE)),
    output:    new Float32Array(BLOCK_SIZE),
    current:   -1,
  };
}

// Returns a Float32Array view over the instance's whole linear memory,
// re-created only when memory.grow() has detached the previous buffer.
function heapFor(instance) {
  const memory = instance.exports.memory;
  let heap = instance.__heap;
  if (!heap || heap.buffer !== memory.buffer) {
    heap = new Float32Array(memory.buffer);
    instance.__heap = heap;
  }
  return heap;
}

function f32Index(ptr) {
  // alloc() hands back a *mut f32; a misaligned pointer would make the
  // index arithmetic below silently read the wrong samples.
  if (ptr & 3) throw new Error(`transform returned a misaligned f32 pointer (${ptr})`);
  return ptr >>> 2;
}

// Copies `ports` and `params` into the instance's memory, runs process(),
// and copies exactly BLOCK_SIZE samples of the result into `out`. A node
// whose module failed to load (instance === null) outputs silence.
export function callWasm(instance, params, ports, out) {
  if (!instance) {
    out.fill(0);
    return;
  }
  const exp = instance.exports;
  const numInputs = ports.length;
  const inputPtr  = exp.alloc(Math.max(numInputs * BLOCK_SIZE, 1));
  const paramsPtr = exp.alloc(Math.max(params.length, 1));

  // alloc() can't grow memory with the SDK's static arena, but re-check the
  // heap after the calls anyway — it's one comparison.
  let heap = heapFor(instance);
  const inputBase = f32Index(inputPtr);
  for (let p = 0; p < numInputs; p++) heap.set(ports[p], inputBase + p * BLOCK_SIZE);
  heap.set(params, f32Index(paramsPtr));

  // ABI v2: process(samples_ptr, num_inputs, block_len, params_ptr, params_len) -> *const f32.
  // samples_ptr points at `num_inputs` contiguous BLOCK_SIZE-sample buffers,
  // one per declared input port in port_order — matches transform-sdk's
  // `Transform::process(samples: &[&[f32]], ...)` contract.
  const outPtr = exp.process(inputPtr, numInputs, BLOCK_SIZE, paramsPtr, params.length);

  // Arena is a fixed-size bump-and-wrap buffer that's never freed (see
  // transform-sdk export.rs alloc()), so the next alloc() call can overwrite
  // outPtr's bytes — copy the output out now, synchronously.
  heap = heapFor(instance);
  const outBase = f32Index(outPtr);
  for (let s = 0; s < BLOCK_SIZE; s++) out[s] = heap[outBase + s];
}

// ─── GraphWorklet ─────────────────────────────────────────────────────────────
//
// Guarded on AudioWorkletProcessor's presence so this module can be imported
// under Vitest (plain Node, no AudioWorkletGlobalScope) to unit-test the
// generated code and runtime helpers directly — see GraphCompiler.test.ts and
// graph-worklet.test.ts. Real worklet loading (via audioWorklet.addModule)
// always runs inside AudioWorkletGlobalScope, where AudioWorkletProcessor is
// defined, so this is a no-op there.

if (typeof AudioWorkletProcessor !== 'undefined') {

const SILENCE = new Float32Array(BLOCK_SIZE);

class GraphWorklet extends AudioWorkletProcessor {
  constructor() {
    super();
    this._bypass       = true;
    this._loading      = false;
    this._faulted      = false;
    this._runtimeGraph = null; // { lanes, params, transformFn, transformIds }
    this._loadToken    = 0;
    this._quantumCount = 0;
    this._hasPerf = typeof performance !== 'undefined';

    new WorkletMessageDispatcher(this.port, this);
  }

  // ── Message handlers ────────────────────────────────────────────────────────

  onSetGraph(compiledGraph, binaries) {
    void this._loadGraph(compiledGraph, binaries);
  }

  onSetBypass(bypass) {
    this._bypass = bypass;
  }

  onUpdateParams(nodeIndex, params) {
    const graph = this._runtimeGraph;
    if (!graph || nodeIndex < 0 || nodeIndex >= graph.params.length) return;
    const current = graph.params[nodeIndex];
    if (current.length === params.length) current.set(params);
    else graph.params[nodeIndex] = new Float32Array(params);
  }

  // ── Graph loading ────────────────────────────────────────────────────────────

  async _loadGraph(compiledGraph, binaries) {
    // A newer SET_GRAPH may arrive while this one is still instantiating;
    // only the latest one is allowed to swap in.
    const token = ++this._loadToken;
    this._loading = true;

    const { executionOrder } = compiledGraph;
    const laneInstances = Array.from({ length: CHANNEL_COUNT }, () => []);
    const params = [];
    const transformIds = [];

    for (const node of executionOrder) {
      transformIds.push(node.transformId);
      params.push(new Float32Array(node.params));

      const binary = binaries[node.transformId];
      let instances = null;
      if (!binary) {
        this.port.postMessage({ type: 'MODULE_ERROR', transformId: node.transformId, error: 'Binary not provided' });
      } else {
        try {
          const module = await WebAssembly.compile(binary);
          instances = [];
          for (let c = 0; c < CHANNEL_COUNT; c++) {
            instances.push(await WebAssembly.instantiate(module));
          }
        } catch (err) {
          this.port.postMessage({ type: 'MODULE_ERROR', transformId: node.transformId, error: String(err) });
          instances = null;
        }
      }
      // Always push one entry per node (null on failure) so instances[i],
      // params[i] and executionOrder[i] stay aligned.
      for (let c = 0; c < CHANNEL_COUNT; c++) {
        laneInstances[c].push(instances ? instances[c] : null);
      }
    }

    if (token !== this._loadToken) return;

    const lanes = laneInstances.map((instances) => createLane(compiledGraph, instances));
    const transformFn = generateTransformFunction(compiledGraph);

    // Atomic swap — process() kept running with the old graph while this loaded.
    this._runtimeGraph = { lanes, params, transformFn, transformIds };
    this._faulted      = false;
    this._loading      = false;

    this.port.postMessage({ type: 'GRAPH_READY' });
  }

  // ── Audio processing ─────────────────────────────────────────────────────────

  process(inputs, outputs) {
    const input  = inputs[0];
    const output = outputs[0];
    if (!output || output.length === 0) return true;

    const graph = this._runtimeGraph;
    const bypass = this._bypass || this._loading || this._faulted || !graph;

    if (bypass) {
      for (let c = 0; c < output.length; c++) {
        output[c].set(input?.[c] ?? SILENCE);
      }
      return true;
    }

    const shouldReportCpu = this._hasPerf && (this._quantumCount % CPU_REPORT_INTERVAL_QUANTA === 0);
    const startTime = shouldReportCpu ? performance.now() : 0;

    const channels = Math.min(output.length, graph.lanes.length);
    let c = 0;
    try {
      for (; c < channels; c++) {
        const result = graph.transformFn(
          input?.[c] ?? SILENCE, graph.lanes[c], graph.params, callWasm, addInto,
        );
        output[c].set(result);
      }
    } catch (err) {
      this._retireGraph(graph.lanes[c], graph.transformIds, err);
      for (let ch = 0; ch < output.length; ch++) {
        output[ch].set(input?.[ch] ?? SILENCE);
      }
      return true;
    }

    if (shouldReportCpu) {
      const elapsedMs = performance.now() - startTime;
      const quantumBudgetMs = (BLOCK_SIZE / sampleRate) * 1000;
      this.port.postMessage({ type: 'CPU_LOAD', value: elapsedMs / quantumBudgetMs });
    }
    this._quantumCount++;

    return true;
  }

  // A WASM trap leaves the instance unusable (see the header comment), so the
  // whole graph is dropped to dry pass-through until the next SET_GRAPH.
  _retireGraph(lane, transformIds, err) {
    this._faulted = true;
    const nodeIndex = lane ? lane.current : -1;
    this.port.postMessage({
      type: 'RUNTIME_ERROR',
      nodeIndex,
      transformId: nodeIndex >= 0 ? transformIds[nodeIndex] : null,
      error: String(err),
    });
  }
}

registerProcessor('graph-worklet', GraphWorklet);

}
