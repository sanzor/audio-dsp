export type FeedbackEdge = {
  fromId: number
  toId: number
  size: number
}

export type FeedbackBuffer = {
  fromId: number
  toId: number
  size: number
}

export type ProcessingPlan = {
  topoOrder: number[]
  nodeOutBuf: Map<number, number>
  regInputs: Map<number, number[]>
  fbInputs: Map<number, number[]>
  graphInputBuf: number
  nextBuf: number
}
