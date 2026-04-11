/**
 * GraphProcessor — AudioWorkletProcessor
 *
 * Runs on the dedicated audio thread.
 *
 * For now all transforms are 1-in 1-out, so the chain is a flat ordered array.
 * The main thread (orchestrator) does the topological sort and sends the result here.
 * Samples flow straight through each module in order.
 *
 * TODO: when mixer nodes (multiple inputs) are added, this becomes a node map
 * with explicit inputNodeIds per entry, and we buffer each node's output to
 * support fan-in. The worklet interface (SET_CHAIN) will change at that point.
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
    this.bypass = true;
    this.chain = [];   // [{ instance: WebAssembly.Instance, params: Float32Array }]
    this.loading = false;

    this.port.onmessage = ({ data }) => {
      if (data.type === 'SET_BYPASS') {
        this.bypass = data.bypass;
      }
      if (data.type === 'SET_CHAIN') {
        void this._loadChain(data.chain);
      }
    };
  }

  async _loadChain(entries) {
    this.loading = true;
    const next = [];

    for (const { binary, params } of entries) {
      try {
        const { instance } = await WebAssembly.instantiate(binary);
        next.push({ instance, params: new Float32Array(params) });
      } catch (err) {
        // one bad module is skipped, rest of chain still loads
        this.port.postMessage({ type: 'MODULE_ERROR', error: String(err) });
      }
    }

    this.chain = next;  // atomic swap
    this.loading = false;
  }

  process(inputs, outputs) {
    const input  = inputs[0]?.[0];
    const output = outputs[0]?.[0];
    if (!input || !output) return true;

    if (this.bypass || this.loading || this.chain.length === 0) {
      output.set(input);
      return true;
    }

    // working buffer — passed through each module in turn
    let samples = new Float32Array(input);

    for (const { instance, params } of this.chain) {
      const exp = instance.exports;

      const ptr = exp.alloc(samples.length);
      new Float32Array(exp.memory.buffer, ptr, samples.length).set(samples);

      const paramsPtr = exp.alloc(params.length);
      new Float32Array(exp.memory.buffer, paramsPtr, params.length).set(params);

      exp.process(ptr, samples.length, paramsPtr, params.length);

      // copy out before next module (WASM memory may be reused)
      samples = Float32Array.from(new Float32Array(exp.memory.buffer, ptr, samples.length));
    }

    output.set(samples);
    return true;
  }
}

registerProcessor('graph-processor', GraphProcessor);
