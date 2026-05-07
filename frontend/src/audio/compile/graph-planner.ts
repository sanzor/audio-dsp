import type { ActiveGraph } from '@/Stores/ActiveGraphState'
import type { FeedbackBuffer, ProcessingPlan } from '../types/processing-plan'

export function buildProcessingPlan(
  graph: ActiveGraph,
  feedbackBuffers: FeedbackBuffer[],
): ProcessingPlan {
  const feedbackSet = new Set(feedbackBuffers.map(b => `${b.fromId}:${b.toId}`))

  const inDegree = new Map<number, number>([...graph.nodes.keys()].map(id => [id, 0]))
  const adjacency = new Map<number, number[]>([...graph.nodes.keys()].map(id => [id, []]))

  for (const edge of graph.edges.values()) {
    if (!feedbackSet.has(`${edge.fromNodeId}:${edge.toNodeId}`)) {
      adjacency.get(edge.fromNodeId)!.push(edge.toNodeId)
      inDegree.set(edge.toNodeId, inDegree.get(edge.toNodeId)! + 1)
    }
  }

  const queue = [...inDegree.entries()].filter(([, d]) => d === 0).map(([id]) => id)
  const topoOrder: number[] = []

  while (queue.length > 0) {
    const nodeId = queue.shift()!
    topoOrder.push(nodeId)

    for (const neighbor of adjacency.get(nodeId)!) {
      const deg = inDegree.get(neighbor)! - 1
      inDegree.set(neighbor, deg)
      if (deg === 0) queue.push(neighbor)
    }
  }

  if (topoOrder.length !== graph.nodes.size) {
    throw new Error('Cycle remains after removing feedback edges')
  }

  const regInputs = new Map<number, number[]>([...graph.nodes.keys()].map(id => [id, []]))
  const fbInputs = new Map<number, number[]>([...graph.nodes.keys()].map(id => [id, []]))

  for (const edge of graph.edges.values()) {
    const key = `${edge.fromNodeId}:${edge.toNodeId}`
    if (feedbackSet.has(key)) {
      const regIdx = feedbackBuffers.findIndex(
        b => b.fromId === edge.fromNodeId && b.toId === edge.toNodeId,
      )
      if (regIdx !== -1) fbInputs.get(edge.toNodeId)!.push(regIdx)
    } else {
      regInputs.get(edge.toNodeId)!.push(edge.fromNodeId)
    }
  }

  const graphInputBuf = 0
  const nodeOutBuf = new Map(topoOrder.map((id, i) => [id, 1 + i]))
  const nextBuf = 1 + topoOrder.length

  return { topoOrder, nodeOutBuf, regInputs, fbInputs, graphInputBuf, nextBuf }
}
