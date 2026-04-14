/**
 * CompiledGraph — the execution plan produced by GraphCompiler.
 *
 * Describes how to route audio through the graph per quantum.
 * No binaries embedded — transformId is used to look up the binary from
 * WasmBinaryStore when instantiating in the worklet.
 *
 * NodeInputSource describes where a node reads its audio from:
 *   'raw'      → the worklet's raw audio input (the track playing)
 *   'buffer'   → current-frame output of an upstream node
 *   'feedback' → previous-frame output of an upstream node (breaks a cycle)
 */

export type NodeInputSource =
  | { kind: 'raw' }
  | { kind: 'buffer';   bufferIndex: number }
  | { kind: 'feedback'; bufferIndex: number };

export interface CompiledNode {
  nodeId: number;
  transformId: number;
  params: number[];             // flattened from Record<string,number>, sent to WASM
  inputs: NodeInputSource[];
  outputBufferIndex: number;    // -1 = sink node: writes additively to worklet output
}

export interface CompiledGraph {
  executionOrder: CompiledNode[];     // sources first, sinks last
  bufferCount: number;                // total Float32Arrays the worklet must allocate
  feedbackBufferIndices: number[];    // which buffers need a prev-frame copy each quantum
}
