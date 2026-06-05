import type { Edge as RFEdge, Node as RFNode } from "reactflow"

export interface RunPipelineParams {
  graphParams: GraphParams
  binaries: Map<number, Uint8Array>
}

export interface GraphParams {
  graphId: number | undefined
  nodes: RFNode[]
  edges: RFEdge[]
}