/**
 * GraphInput — what the compiler receives.
 *
 * Pure structural description of the graph: topology + params.
 * No binaries here — those live in WasmBinaryStore keyed by transformId.
 */

export interface GraphInputNode {
  id: number;
  transformId: number;
  params: Record<string, number>; // knob values set by the user in the UI (e.g. { decay: 0.8 })
}

export interface GraphInputEdge {
  id: number;
  fromNodeId: number;
  toNodeId: number;
}

export interface GraphInput {
  nodes: Map<number, GraphInputNode>;
  edges: Map<number, GraphInputEdge>;
}
