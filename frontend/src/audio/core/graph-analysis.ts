import type { ActiveGraph } from '../types/graph'
import type { FeedbackBuffer, FeedbackEdge } from '../types/processing-plan'

export function getFeedbackEdges(graph: ActiveGraph): FeedbackEdge[] {
  const feedbackEdges: FeedbackEdge[] = []
  const visited = new Set<number>()
  const visiting = new Set<number>()

  for (const nodeId of graph.nodes.keys()) {
    if (!visited.has(nodeId)) {
      dfs(nodeId, graph, visited, visiting, feedbackEdges)
    }
  }

  return feedbackEdges
}

function dfs(
  nodeId: number,
  graph: ActiveGraph,
  visited: Set<number>,
  visiting: Set<number>,
  feedbackEdges: FeedbackEdge[],
): void {
  visiting.add(nodeId)

  const neighbors = [...graph.edges.values()]
    .filter(e => e.fromNodeId === nodeId)
    .map(e => e.toNodeId)

  for (const target of neighbors) {
    if (visiting.has(target)) {
      feedbackEdges.push({ fromId: nodeId, toId: target, size: 128 })
    } else if (!visited.has(target)) {
      dfs(target, graph, visited, visiting, feedbackEdges)
    }
  }

  visiting.delete(nodeId)
  visited.add(nodeId)
}

export function createFeedbackBuffers(edges: FeedbackEdge[]): FeedbackBuffer[] {
  return edges.map(e => ({ fromId: e.fromId, toId: e.toId, size: e.size }))
}
