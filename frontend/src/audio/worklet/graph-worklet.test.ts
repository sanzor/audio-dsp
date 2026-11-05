import { describe, it, expect, vi, beforeAll, afterEach } from "vitest";
import type { CompiledGraph } from "@/audio/pipeline/compile-graph/compiledGraph";
import type { CompiledNode, NodeInputSource } from "@/audio/pipeline/GraphCompiler";

// Drives the real GraphWorklet processor under plain Node by stubbing the
// AudioWorkletGlobalScope pieces it touches (AudioWorkletProcessor,
// registerProcessor, sampleRate) and WebAssembly.compile/instantiate, so the
// WASM side is a hand-written fake that speaks the transform-sdk ABI:
// alloc(len) -> *mut f32 in a bump arena, process(...) -> *const f32.

const BLOCK = 128;

type Processor = {
  port: { postMessage: ReturnType<typeof vi.fn> };
  onSetGraph: (graph: CompiledGraph, binaries: Record<number, ArrayBuffer>) => void;
  onSetBypass: (bypass: boolean) => void;
  onUpdateParams: (nodeIndex: number, params: number[]) => void;
  process: (inputs: Float32Array[][], outputs: Float32Array[][]) => boolean;
};

let ProcessorClass: new () => Processor;

beforeAll(async () => {
  class FakeAudioWorkletProcessor {
    port = { postMessage: vi.fn(), onmessage: null as unknown };
  }
  const g = globalThis as Record<string, unknown>;
  g.AudioWorkletProcessor = FakeAudioWorkletProcessor;
  g.sampleRate = 48000;
  g.registerProcessor = (_name: string, cls: new () => Processor) => {
    ProcessorClass = cls;
  };
  await import("./graph-worklet.js");
});

afterEach(() => {
  vi.restoreAllMocks();
});

// ─── Fake transform-sdk module ────────────────────────────────────────────────

type ProcessFn = (inputs: Float32Array[], params: Float32Array, state: { n: number }) => number[] | Float32Array;

function fakeInstance(processFn: ProcessFn) {
  const memory = { buffer: new ArrayBuffer(1 << 16) };
  const heap = new Float32Array(memory.buffer);
  let cursor = 0;
  const state = { n: 0 };
  const alloc = (len: number) => {
    const bytes = len * 4;
    if (cursor + bytes > memory.buffer.byteLength) cursor = 0;
    const ptr = cursor;
    cursor += bytes;
    return ptr;
  };
  return {
    state,
    exports: {
      memory,
      alloc,
      process(samplesPtr: number, numInputs: number, blockLen: number, paramsPtr: number, paramsLen: number) {
        const inputs = Array.from({ length: numInputs }, (_, i) =>
          heap.slice(samplesPtr / 4 + i * blockLen, samplesPtr / 4 + (i + 1) * blockLen),
        );
        const params = heap.slice(paramsPtr / 4, paramsPtr / 4 + paramsLen);
        const result = processFn(inputs, params, state);
        const outPtr = alloc(blockLen);
        for (let s = 0; s < blockLen; s++) heap[outPtr / 4 + s] = result[s] ?? 0;
        return outPtr;
      },
    },
  };
}

// Each transformId maps to a ProcessFn; every instantiation gets fresh state
// (like a real WebAssembly.instantiate gets fresh linear memory).
function stubWasm(byTransform: Record<number, ProcessFn>) {
  const created: ReturnType<typeof fakeInstance>[] = [];
  vi.spyOn(WebAssembly, "compile").mockImplementation(async (bytes) => {
    const id = new Uint8Array(bytes as ArrayBuffer)[0];
    return { id } as unknown as WebAssembly.Module;
  });
  vi.spyOn(WebAssembly, "instantiate").mockImplementation((async (module: { id: number }) => {
    const inst = fakeInstance(byTransform[module.id]);
    created.push(inst);
    return inst;
  }) as unknown as typeof WebAssembly.instantiate);
  return created;
}

function binaryFor(transformId: number): ArrayBuffer {
  return new Uint8Array([transformId]).buffer;
}

function node(transformId: number, inputs: NodeInputSource[][], extra: Partial<CompiledNode> = {}): CompiledNode {
  return {
    nodeId: transformId,
    transformId,
    params: [],
    inputs,
    outputBufferIndex: -1,
    writesToOutput: true,
    ...extra,
  };
}

function block(value: number | ((i: number) => number)): Float32Array {
  const b = new Float32Array(BLOCK);
  for (let i = 0; i < BLOCK; i++) b[i] = typeof value === "number" ? value : value(i);
  return b;
}

function stereoOut(): Float32Array[][] {
  return [[new Float32Array(BLOCK), new Float32Array(BLOCK)]];
}

async function loadedProcessor(graph: CompiledGraph, binaries: Record<number, ArrayBuffer>) {
  const p = new ProcessorClass();
  p.onSetBypass(false);
  p.onSetGraph(graph, binaries);
  await vi.waitFor(() => {
    const types = p.port.postMessage.mock.calls.map((c) => c[0].type);
    expect(types).toContain("GRAPH_READY");
  });
  return p;
}

const gain = (factor: number): ProcessFn => (inputs) => Array.from(inputs[0], (s) => s * factor);

// ─── Tests ───────────────────────────────────────────────────────────────────

describe("GraphWorklet stereo processing", () => {
  it("processes left and right channels independently instead of dropping the right channel", async () => {
    stubWasm({ 1: gain(2) });
    const p = await loadedProcessor(
      { executionOrder: [node(1, [[{ kind: "raw" }]])], bufferCount: 0, feedbackBufferIndices: [] },
      { 1: binaryFor(1) },
    );

    const out = stereoOut();
    p.process([[block(0.1), block(0.3)]], out);

    expect(out[0][0][0]).toBeCloseTo(0.2);
    expect(out[0][1][0]).toBeCloseTo(0.6);
  });

  it("gives each channel its own instance so stateful transforms don't interleave L/R state", async () => {
    // Counts process() calls on its own instance and outputs that count.
    const created = stubWasm({ 1: (_inputs, _params, state) => block(++state.n) });
    const p = await loadedProcessor(
      { executionOrder: [node(1, [[{ kind: "raw" }]])], bufferCount: 0, feedbackBufferIndices: [] },
      { 1: binaryFor(1) },
    );

    const out = stereoOut();
    p.process([[block(0), block(0)]], out);
    p.process([[block(0), block(0)]], out);

    expect(created).toHaveLength(2);
    // With one shared instance this would read 3 and 4.
    expect(out[0][0][0]).toBe(2);
    expect(out[0][1][0]).toBe(2);
  });

  it("treats a disconnected input as silence instead of throwing", async () => {
    stubWasm({ 1: (inputs) => Array.from(inputs[0], (s) => s + 0.25) });
    const p = await loadedProcessor(
      { executionOrder: [node(1, [[{ kind: "raw" }]])], bufferCount: 0, feedbackBufferIndices: [] },
      { 1: binaryFor(1) },
    );

    const out = stereoOut();
    p.process([[]], out);

    expect(out[0][0][0]).toBeCloseTo(0.25);
    expect(p.port.postMessage.mock.calls.map((c) => c[0].type)).not.toContain("RUNTIME_ERROR");
  });
});

describe("GraphWorklet fault isolation", () => {
  it("retires the graph to dry pass-through and reports RUNTIME_ERROR once when a transform traps", async () => {
    stubWasm({
      1: gain(2),
      2: () => {
        throw new WebAssembly.RuntimeError("unreachable");
      },
    });
    const p = await loadedProcessor(
      {
        executionOrder: [
          node(1, [[{ kind: "raw" }]], { outputBufferIndex: 0, writesToOutput: false }),
          node(2, [[{ kind: "buffer", bufferIndex: 0 }]]),
        ],
        bufferCount: 1,
        feedbackBufferIndices: [],
      },
      { 1: binaryFor(1), 2: binaryFor(2) },
    );

    const out = stereoOut();
    expect(() => p.process([[block(0.1), block(0.3)]], out)).not.toThrow();
    p.process([[block(0.1), block(0.3)]], out);

    const errors = p.port.postMessage.mock.calls.map((c) => c[0]).filter((m) => m.type === "RUNTIME_ERROR");
    expect(errors).toHaveLength(1);
    expect(errors[0]).toMatchObject({ nodeIndex: 1, transformId: 2 });
    expect(errors[0].error).toContain("unreachable");
    // Dry signal on both channels, not silence and not a half-processed block.
    expect(out[0][0][0]).toBeCloseTo(0.1);
    expect(out[0][1][0]).toBeCloseTo(0.3);
  });

  it("recovers on the next SET_GRAPH", async () => {
    let shouldTrap = true;
    stubWasm({
      1: (inputs) => {
        if (shouldTrap) throw new WebAssembly.RuntimeError("unreachable");
        return Array.from(inputs[0], (s) => s * 2);
      },
    });
    const graph: CompiledGraph = { executionOrder: [node(1, [[{ kind: "raw" }]])], bufferCount: 0, feedbackBufferIndices: [] };
    const p = await loadedProcessor(graph, { 1: binaryFor(1) });
    p.process([[block(0.1), block(0.1)]], stereoOut());

    shouldTrap = false;
    p.port.postMessage.mockClear();
    p.onSetGraph(graph, { 1: binaryFor(1) });
    await vi.waitFor(() => expect(p.port.postMessage.mock.calls.map((c) => c[0].type)).toContain("GRAPH_READY"));

    const out = stereoOut();
    p.process([[block(0.1), block(0.1)]], out);
    expect(out[0][0][0]).toBeCloseTo(0.2);
  });

  it("keeps later nodes aligned with their own instance when an earlier binary is missing", async () => {
    stubWasm({ 2: gain(3) });
    const p = await loadedProcessor(
      {
        executionOrder: [
          node(1, [[{ kind: "raw" }]], { writesToOutput: false }),
          node(2, [[{ kind: "raw" }]]),
        ],
        bufferCount: 0,
        feedbackBufferIndices: [],
      },
      { 2: binaryFor(2) }, // transform 1's binary is missing
    );

    const out = stereoOut();
    p.process([[block(0.1), block(0.1)]], out);

    const types = p.port.postMessage.mock.calls.map((c) => c[0].type);
    expect(types).toContain("MODULE_ERROR");
    expect(types).not.toContain("RUNTIME_ERROR");
    // Node 2 still runs its own gain(3); a shifted index would run nothing.
    expect(out[0][0][0]).toBeCloseTo(0.3);
  });
});

describe("GraphWorklet real-time safety", () => {
  it("does not allocate typed arrays inside process()", async () => {
    stubWasm({ 1: gain(2), 2: gain(0.5) });
    const p = await loadedProcessor(
      {
        executionOrder: [
          node(1, [[{ kind: "raw" }, { kind: "feedback", bufferIndex: 1 }]], { outputBufferIndex: 0, writesToOutput: false }),
          node(2, [[{ kind: "buffer", bufferIndex: 0 }], []], { outputBufferIndex: 1, writesToOutput: true }),
        ],
        bufferCount: 2,
        feedbackBufferIndices: [1],
      },
      { 1: binaryFor(1), 2: binaryFor(2) },
    );
    const inputs = [[block(0.1), block(0.2)]];
    const out = stereoOut();
    p.process(inputs, out); // warm-up: caches each instance's heap view

    // The fake WASM itself allocates (heap.slice) — exclude it by only
    // counting Float32Array constructions outside its process().
    let worklet = 0;
    let insideWasm = false;
    const RealF32 = globalThis.Float32Array;
    const Counting = new Proxy(RealF32, {
      construct(target, args) {
        if (!insideWasm) worklet++;
        return Reflect.construct(target, args);
      },
    });
    globalThis.Float32Array = Counting as Float32ArrayConstructor;
    const origSlice = RealF32.prototype.slice;
    RealF32.prototype.slice = function (this: Float32Array, ...args: [number?, number?]) {
      insideWasm = true;
      try {
        return origSlice.apply(this, args);
      } finally {
        insideWasm = false;
      }
    };
    try {
      for (let q = 0; q < 10; q++) p.process(inputs, out);
    } finally {
      globalThis.Float32Array = RealF32;
      RealF32.prototype.slice = origSlice;
    }

    expect(worklet).toBe(0);
  });

  it("delays feedback by exactly one quantum and sums multiple sources into one port", async () => {
    // node 1: in = raw + feedback(node 2's previous output); node 2: identity.
    stubWasm({ 1: gain(1), 2: gain(1) });
    const p = await loadedProcessor(
      {
        executionOrder: [
          node(1, [[{ kind: "raw" }, { kind: "feedback", bufferIndex: 1 }]], { outputBufferIndex: 0, writesToOutput: false }),
          node(2, [[{ kind: "buffer", bufferIndex: 0 }]], { outputBufferIndex: 1, writesToOutput: true }),
        ],
        bufferCount: 2,
        feedbackBufferIndices: [1],
      },
      { 1: binaryFor(1), 2: binaryFor(2) },
    );

    const out = stereoOut();
    p.process([[block(1), block(1)]], out);
    expect(out[0][0][0]).toBe(1); // no feedback yet
    p.process([[block(1), block(1)]], out);
    expect(out[0][0][0]).toBe(2); // 1 + previous 1
    p.process([[block(1), block(1)]], out);
    expect(out[0][0][0]).toBe(3);
  });

  it("keeps separate ports separate (program and sidechain are not mixed)", async () => {
    // Outputs port0 - port1, so mixing would show up as 0.
    stubWasm({ 1: (inputs) => Array.from(inputs[0], (s, i) => s - inputs[1][i]) });
    const p = await loadedProcessor(
      {
        executionOrder: [node(1, [[{ kind: "raw" }], []])],
        bufferCount: 0,
        feedbackBufferIndices: [],
      },
      { 1: binaryFor(1) },
    );

    const out = stereoOut();
    p.process([[block(0.5), block(0.5)]], out);
    expect(out[0][0][0]).toBeCloseTo(0.5); // unwired sidechain is silence
  });
});
