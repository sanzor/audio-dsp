import type { CompiledNode } from "../GraphCompiler";

export interface CompiledGraph {
  executionOrder: CompiledNode[];     // sources first, sinks last
  bufferCount: number;                // total Float32Arrays the worklet must allocate
  feedbackBufferIndices: number[];    // which buffers need a prev-frame copy each quantum
}
