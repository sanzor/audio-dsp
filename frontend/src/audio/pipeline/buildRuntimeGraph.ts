import type { Edge as RFEdge, Node as RFNode } from "reactflow"
import type { ActiveEdge, ActiveGraph, ActiveNode } from "@/Stores/ActiveGraphState"

function toNumericId(raw: string | number): number | null {
  const numeric = Number(raw)
  return Number.isFinite(numeric) ? numeric : null
}

export function buildRuntimeGraph(
  graphId: number | undefined,
  nodes: RFNode[],
  edges: RFEdge[],
): ActiveGraph | null {
  const sourceIds = new Set<number>()
  const sinkIds = new Set<number>()
  const runtimeNodes = new Map<number, ActiveNode>()

  for (const node of nodes) {
    const nodeId = toNumericId(node.id)
    if (nodeId == null) continue

    const nodeType = node.data.nodeType as string | undefined
    if (nodeType === "source") { sourceIds.add(nodeId); continue }
    if (nodeType === "sink") { sinkIds.add(nodeId); continue }

    const transformId = node.data.transformId as number | null | undefined
    if (transformId == null) continue

    runtimeNodes.set(nodeId, {
      id: nodeId,
      transformId,
      position: { x: 0, y: 0 },
      params: (node.data.params as Record<string, number> | undefined) ?? {},
      binary: null,
      binaryStatus: "idle",
      binaryError: null,
    })
  }

  if (runtimeNodes.size === 0) return null

  const runtimeEdges = new Map<number, ActiveEdge>()
  for (const edge of edges) {
    const edgeId = toNumericId(edge.id)
    const fromNodeId = toNumericId(edge.source)
    const toNodeId = toNumericId(edge.target)
    if (edgeId == null || fromNodeId == null || toNodeId == null) continue

    if (
      sourceIds.has(fromNodeId) || sinkIds.has(toNodeId) ||
      sourceIds.has(toNodeId) || sinkIds.has(fromNodeId)
    ) continue

    if (!runtimeNodes.has(fromNodeId) || !runtimeNodes.has(toNodeId)) continue

    runtimeEdges.set(edgeId, { id: edgeId, fromNodeId, toNodeId, fromPortId: 0, toPortId: 0 })
  }

  return {
    id: graphId ?? null,
    regionId: null,
    name: "Live Canvas Graph",
    nodes: runtimeNodes,
    edges: runtimeEdges,
    isDirty: false,
    enabled: true,
  }
}
