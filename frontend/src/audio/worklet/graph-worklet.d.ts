// Ambient type declarations for graph-worklet.js's exports meant for
// non-worklet consumption: unit tests import the code generator and runtime
// helpers directly (GraphCompiler.test.ts, graph-worklet.test.ts). The
// processor itself is loaded at runtime via audioWorklet.addModule(URL) (see
// useWorkletSetup.ts / creatorTransformPlayback.ts), never imported as a
// module elsewhere.
import type { CompiledGraph } from "../pipeline/compile-graph/compiledGraph";

export const BLOCK_SIZE: number;
export const CHANNEL_COUNT: number;

export interface WasmLikeInstance {
  exports: {
    memory: { buffer: ArrayBuffer };
    alloc: (len: number) => number;
    process: (
      samplesPtr: number,
      numInputs: number,
      blockLen: number,
      paramsPtr: number,
      paramsLen: number,
    ) => number;
  };
}

export interface Lane {
  instances: (WasmLikeInstance | null)[];
  buffers: Float32Array[];
  prev: Float32Array[];
  ports: Float32Array[][];
  portLists: Float32Array[][];
  outs: Float32Array[];
  output: Float32Array;
  current: number;
}

export type CallWasm = (
  instance: WasmLikeInstance | null,
  params: Float32Array,
  ports: Float32Array[],
  out: Float32Array,
) => void;

export type TransformFn = (
  rawAudio: Float32Array,
  lane: Lane,
  params: Float32Array[],
  callWasm: CallWasm,
  addInto: (target: Float32Array, source: Float32Array) => void,
) => Float32Array;

export function generateTransformFunction(compiledGraph: CompiledGraph): TransformFn;
export function createLane(
  compiledGraph: CompiledGraph,
  instances: (WasmLikeInstance | null)[],
): Lane;
export function addInto(target: Float32Array, source: Float32Array): void;
export const callWasm: CallWasm;
