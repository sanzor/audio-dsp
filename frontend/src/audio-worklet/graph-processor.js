/**
 * GraphProcessor — AudioWorkletProcessor
 *
 * Runs on the dedicated audio thread.
 *
 * For now all transforms are 1-in 1-out, so the chain is a flat ordered array.
 * The main thread (orchestrator) does the topological sort and sends the result here.
 * Samples flow straight through each module in order.
 *
 * Messages received from the main thread:
 *
 *   SET_BYPASS { bypass: boolean }
 *     When true, samples pass through unchanged. Used for the enable/disable toggle.
 *
 *   SET_CHAIN { chain: Array<{ binary: Uint8Array, params: number[] }> }
 *     Replaces the transform chain. Already in execution order.
 *     We instantiate all modules then atomically swap — process() keeps running
 *     with the old chain while the new one loads.
 *
 * Each Rust WASM module must export:
 *   alloc(len: i32) -> i32
 *   process(ptr: i32, len: i32, params_ptr: i32, params_len: i32) -> void
 *   memory: WebAssembly.Memory
 */

class GraphProcessor extends AudioWorkletProcessor {
  constructor() {
    super();
    this.bypass  = true;
    this.graph   = null;
    this.nodes   = []; // Processed nodes with pre-allocated memory pointers
    this.buffers = []; // Intermediate Float32Array buffers
    this.feedbackBuffers = [];
    this.loading = false;

    this.port.onmessage = ({ data }) => {
      if (data.type === 'SET_BYPASS') {
        this.bypass = data.bypass;
      }
      if (data.type === 'SET_GRAPH') {
        void this._loadGraph(data);
      }
      if (data.type === 'UPDATE_PARAMS') {
        this._updateNodeParams(data.nodeIndex, data.params);
      }
    };
  }

  async _loadGraph({ graph, binaries }) {
    this.loading = true;
    const nextNodes = [];
    const instances = new Map();

    try {
      // 1. Instantiate all required modules
      for (const [idStr, bin] of Object.entries(binaries)) {
        const { instance } = await WebAssembly.instantiate(bin);
        instances.set(Number(idStr), instance);
      }

      // 2. Prepare nodes and pre-allocate memory pointers
      for (const node of graph.executionOrder) {
        const instance = instances.get(node.transformId);
        const exp = instance.exports;

        // Pre-allocate buffer pointers to avoid calling alloc() in the audio loop
        const inputPtr  = exp.alloc(128); 
        const paramsPtr = exp.alloc(node.params.length);

        // Initial parameter sync
        new Float32Array(exp.memory.buffer, paramsPtr, node.params.length).set(node.params);

        nextNodes.push({
          instance,
          inputPtr,
          paramsPtr,
          paramsLen: node.params.length,
          inputs:    node.inputs,
          outputBufferIndex: node.outputBufferIndex
        });
      }

      // 3. Allocate intermediate routing buffers
      this.buffers = Array.from({ length: graph.bufferCount }, () => new Float32Array(128));
      this.feedbackBuffers = Array.from({ length: graph.bufferCount }, () => new Float32Array(128));
      
      this.graph = graph;
      this.nodes = nextNodes;
    } catch (err) {
      this.port.postMessage({ type: 'MODULE_ERROR', error: String(err) });
    } finally {
      this.loading = false;
    }
  }

  _updateNodeParams(nodeIndex, params) {
    const node = this.nodes[nodeIndex];
    if (!node) return;
    const exp = node.instance.exports;
    new Float32Array(exp.memory.buffer, node.paramsPtr, node.paramsLen).set(params);
  }

  process(inputs, outputs) {
    const input  = inputs[0]?.[0];
    const output = outputs[0]?.[0];
    if (!input || !output) return true;

    if (this.bypass || this.loading || this.nodes.length === 0) {
      output.set(input);
      return true;
    }

    // 1. Clear output (since sinks will additively mix into it)
    output.fill(0);

    // 2. Process nodes in topological order
    for (const node of this.nodes) {
      const { instance, inputPtr, paramsPtr, paramsLen, inputs: sources, outputBufferIndex } = node;
      const exp = instance.exports;

      // Sum inputs into a local buffer
      const mixedInput = new Float32Array(128);
      for (const src of sources) {
        if (src.kind === 'raw') {
          for (let i = 0; i < 128; i++) mixedInput[i] += input[i];
        } else if (src.kind === 'buffer') {
          const b = this.buffers[src.bufferIndex];
          for (let i = 0; i < 128; i++) mixedInput[i] += b[i];
        } else if (src.kind === 'feedback') {
          const b = this.feedbackBuffers[src.bufferIndex];
          for (let i = 0; i < 128; i++) mixedInput[i] += b[i];
        }
      }

      // Write to WASM and Process
      new Float32Array(exp.memory.buffer, inputPtr, 128).set(mixedInput);
      exp.process(inputPtr, 128, paramsPtr, paramsLen);

      // Read result
      const result = new Float32Array(exp.memory.buffer, inputPtr, 128);

      if (outputBufferIndex === -1) {
        // Sink node
        for (let i = 0; i < 128; i++) output[i] += result[i];
      } else {
        this.buffers[outputBufferIndex].set(result);
      }
    }

    // 3. Update feedback buffers for the next frame
    for (const idx of this.graph.feedbackBufferIndices) {
      this.feedbackBuffers[idx].set(this.buffers[idx]);
    }

    return true;
  }
}

registerProcessor('graph-processor', GraphProcessor);
