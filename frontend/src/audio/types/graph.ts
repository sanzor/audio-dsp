export type BinaryStatus = 'idle' | 'loading' | 'ready' | 'error'

export type Position = { x: number; y: number }

export type ActiveNode = {
  id: number
  transformId: number
  position: Position
  params: Record<string, number>
  binary: ArrayBuffer | null
  binaryStatus: BinaryStatus
  binaryError: string | null
}

export type ActiveEdge = {
  id: number
  fromNodeId: number
  toNodeId: number
  fromPortId: number
  toPortId: number
}

export type ActiveGraph = {
  id: number | null
  regionId: number | null
  name: string
  nodes: Map<number, ActiveNode>
  edges: Map<number, ActiveEdge>
  isDirty: boolean
  enabled: boolean
}
