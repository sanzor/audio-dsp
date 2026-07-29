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
 *   CPU_LOAD      — throttled processing-time-vs-quantum-budget ratio
 *
 * On SET_GRAPH the worklet:
 *   1. Instantiates WASM modules for each node
 *   2. Generates a dedicated transform function from the graph topology
 *      (no interpretation loop at runtime — flat sequential calls)
 *   3. Atomically swaps in the new graph
 *
 * NOTE: new Function() is used for code generation. Ensure your CSP allows
 * 'unsafe-eval' in the worker-src / script-src directives.
 */

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

function generateTransformFunction(compiledGraph) {
  const { executionOrder } = compiledGraph;
  const lines = [];

  lines.push('let output = SIL();');

  for (let i = 0; i < executionOrder.length; i++) {
    const node = executionOrder[i];

    // Gather all input sources for this node into named variables, then sum them.
    const inputVars = node.inputs.map((src, j) => {
      const varName = `in_${i}_${j}`;
      if (src.kind === 'raw') {
        lines.push(`const ${varName} = rawAudio;`);
      } else if (src.kind === 'feedback') {
        lines.push(`const ${varName} = prev.get(${src.bufferIndex}) ?? SIL();`);
      } else {
        lines.push(`const ${varName} = buffers[${src.bufferIndex}];`);
      }
      return varName;
    });

    const summedInput = inputVars.length === 1
      ? inputVars[0]
      : `addAll(${inputVars.join(', ')})`;

    lines.push(`const out_${i} = callWasm(instances[${i}], params[${i}], ${summedInput});`);

    if (node.outputBufferIndex === -1) {
      lines.push(`output = addAll(output, out_${i});`);
    } else {
      lines.push(`buffers[${node.outputBufferIndex}].set(out_${i});`);
    }
  }

  // Snapshot loop-edge buffers for the next quantum.
  const loopEdgeIndices = compiledGraph.feedbackBufferIndices ?? [];
  lines.push('const nextPrev = new Map();');
  for (const idx of loopEdgeIndices) {
    lines.push(`nextPrev.set(${idx}, buffers[${idx}].slice());`);
  }

  lines.push('return { output, nextPrev };');

  return new Function(
    'rawAudio', 'instances', 'params', 'buffers', 'prev',
    'SIL', 'addAll', 'callWasm',
    lines.join('\n'),
  );
}

// ─── Runtime helpers (passed into the generated function) ─────────────────────

const BLOCK_SIZE = 128;
// Report CPU load every N quanta rather than every quantum (~370/sec at 48kHz)
// to avoid flooding the message port.
const CPU_REPORT_INTERVAL_QUANTA = 20;

const SIL    = () => new Float32Array(BLOCK_SIZE);

function addAll(...buffers) {
  const result = new Float32Array(BLOCK_SIZE);
  for (const buf of buffers) for (let i = 0; i < BLOCK_SIZE; i++) result[i] += buf[i];
  return result;
}

function callWasm(instance, params, input) {
  const exp = instance.exports;
  const blockLen  = input.length;
  const inputPtr  = exp.alloc(blockLen);
  const paramsPtr = exp.alloc(Math.max(params.length, 1));
  new Float32Array(exp.memory.buffer, inputPtr,  blockLen      ).set(input);
  new Float32Array(exp.memory.buffer, paramsPtr, params.length ).set(params);
  // ABI v2: process(samples_ptr, num_inputs, block_len, params_ptr, params_len) -> *const f32.
  // num_inputs is always 1 today — generateTransformFunction already sums every
  // node's upstream sources into one buffer before calling this.
  const outPtr = exp.process(inputPtr, 1, blockLen, paramsPtr, params.length);
  // Arena is a fixed-size bump-and-wrap buffer that's never freed (see
  // transform-sdk export.rs alloc()), so the next alloc() call can overwrite
  // outPtr's bytes — materialize the output now, synchronously.
  return Float32Array.from(new Float32Array(exp.memory.buffer, outPtr, blockLen));
}

// ─── GraphWorklet ─────────────────────────────────────────────────────────────

class GraphWorklet extends AudioWorkletProcessor {
  constructor() {
    super();
    this._bypass      = true;
    this._loading     = false;
    this._runtimeGraph = null; // { instances, params, buffers, transformFn, loopEdgeIndices }
    this._previousFrame = new Map();
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
    if (!this._runtimeGraph) return;
    this._runtimeGraph.params[nodeIndex] = new Float32Array(params);
  }

  // ── Graph loading ────────────────────────────────────────────────────────────

  async _loadGraph(compiledGraph, binaries) {
    this._loading = true;

    const instances = [];
    const params    = [];

    for (const node of compiledGraph.executionOrder) {
      const binary = binaries[node.transformId];
      if (!binary) {
        this.port.postMessage({ type: 'MODULE_ERROR', transformId: node.transformId, error: 'Binary not provided' });
        continue;
      }
      try {
        const { instance } = await WebAssembly.instantiate(binary);
        instances.push(instance);
        params.push(new Float32Array(node.params));
      } catch (err) {
        this.port.postMessage({ type: 'MODULE_ERROR', transformId: node.transformId, error: String(err) });
        instances.push(null); // keep indices aligned
        params.push(new Float32Array(0));
      }
    }

    const buffers    = Array.from({ length: compiledGraph.bufferCount }, () => new Float32Array(BLOCK_SIZE));
    const transformFn = generateTransformFunction(compiledGraph);

    // Atomic swap — process() keeps running with old graph while this was loading.
    this._runtimeGraph  = { instances, params, buffers, transformFn };
    this._previousFrame = new Map();
    this._loading       = false;

    this.port.postMessage({ type: 'GRAPH_READY' });
  }

  // ── Audio processing ─────────────────────────────────────────────────────────

  process(inputs, outputs) {
    const rawAudio  = inputs[0]?.[0];
    const rawOutput = outputs[0]?.[0];
    if (!rawOutput) return true;

    const bypass = this._bypass || this._loading || !this._runtimeGraph;
    if (bypass) {
      rawOutput.set(rawAudio ?? SIL());
      return true;
    }

    const { instances, params, buffers, transformFn } = this._runtimeGraph;

    const shouldReportCpu = this._hasPerf && (this._quantumCount % CPU_REPORT_INTERVAL_QUANTA === 0);
    const startTime = shouldReportCpu ? performance.now() : 0;

    const { output, nextPrev } = transformFn(
      rawAudio, instances, params, buffers, this._previousFrame,
      SIL, addAll, callWasm,
    );

    if (shouldReportCpu) {
      const elapsedMs = performance.now() - startTime;
      const quantumBudgetMs = (BLOCK_SIZE / sampleRate) * 1000;
      this.port.postMessage({ type: 'CPU_LOAD', value: elapsedMs / quantumBudgetMs });
    }
    this._quantumCount++;

    this._previousFrame = nextPrev; // impure assignment — lives here, not inside the generated fn
    rawOutput.set(output);

    return true;
  }
}

registerProcessor('graph-worklet', GraphWorklet);
