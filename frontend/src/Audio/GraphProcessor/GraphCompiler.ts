/**
 * GraphCompiler
 *
 * Pure function: GraphInput → CompileResult.  No side effects, no I/O.
 *
 * Phases
 * ──────
 * 1. findBackEdges       — DFS; marks edges that close a cycle (back-edges).
 *                          A back-edge points to an ancestor still on the DFS
 *                          stack, meaning "this node feeds audio back to an
 *                          earlier node."  We break the cycle at runtime by
 *                          using the upstream node's output from the *previous*
 *                          audio frame instead of the current one.
 *
 * 2. topoSort            — Kahn's BFS on the forward-edge DAG (back-edges
 *                          excluded).  Produces an order where every node's
 *                          upstream dependencies are already processed.
 *
 * 3. assignOutputBuffers — Gives each non-sink node one numbered buffer slot
 *                          (a Float32Array the worklet allocates).  A node
 *                          writes its output once into its slot; every
 *                          downstream node reads from the same slot (fan-out
 *                          is free).  Sink nodes (nothing downstream) write
 *                          straight to the worklet output — no slot needed.
 *                          Nodes that also have back-edges out are flagged so
 *                          the worklet knows to save their slot at frame end.
 *
 * 4. buildNodes          — Assembles the final CompiledNode list in execution
 *                          order, wiring up which slots each node reads from
 *                          and which slot (or the raw output) it writes to.
 */

import type { GraphInput, GraphInputEdge } from './dto/GraphInput';
import type { CompiledGraph, CompiledNode, NodeInputSource } from './dto/CompiledGraph';

// ─── Result type ──────────────────────────────────────────────────────────────

export type CompileResult =
  | { ok: true;  graph: CompiledGraph }
  | { ok: false; reason: 'empty_graph' };

// ─── Entry point ──────────────────────────────────────────────────────────────

export function process(input: GraphInput): CompileResult {
  const { nodes, edges } = input;

  if (nodes.size === 0) {
    return { ok: false, reason: 'empty_graph' };
  }

  const backEdgeIds      = findBackEdges(nodes, edges);
  const executionOrder   = topoSort(nodes, edges, backEdgeIds);
  const bufferMap        = assignOutputBuffers(executionOrder, edges, backEdgeIds);
  const compiledNodes    = buildNodes(executionOrder, nodes, edges, backEdgeIds, bufferMap);

  return {
    ok: true,
    graph: {
      executionOrder:        compiledNodes,
      bufferCount:           bufferMap.bufferCount,
      feedbackBufferIndices: bufferMap.feedbackBufferIndices,
    },
  };
}

// ─── Phase 1: find back-edges ─────────────────────────────────────────────────

function findBackEdges(
  nodes: GraphInput['nodes'],
  edges: GraphInput['edges'],
): Set<number> {
  const outEdges    = buildEdgesByFrom(nodes, edges);
  const backEdgeIds = new Set<number>();

  const WHITE = 0, GRAY = 1, BLACK = 2;
  const color = new Map<number, 0 | 1 | 2>(
    [...nodes.keys()].map((id) => [id, WHITE]),
  );

  const visit = (id: number) => {
    color.set(id, GRAY); // on the DFS stack
    for (const edge of outEdges.get(id) ?? []) {
      const neighborColor = color.get(edge.toNodeId) ?? WHITE;
      if (neighborColor === GRAY)       backEdgeIds.add(edge.id); // ancestor → cycle
      else if (neighborColor === WHITE) visit(edge.toNodeId);
    }
    color.set(id, BLACK); // fully explored, off the stack
  };

  for (const id of nodes.keys()) {
    if ((color.get(id) ?? WHITE) === WHITE) visit(id);
  }

  return backEdgeIds;
}

// ─── Phase 2: topological sort ───────────────────────────────────────────────

function topoSort(
  nodes: GraphInput['nodes'],
  edges: GraphInput['edges'],
  backEdgeIds: Set<number>,
): number[] {
  const outEdges = buildEdgesByFrom(nodes, edges);

  const inDegree = new Map<number, number>(
    [...nodes.keys()].map((id) => [id, 0]),
  );
  for (const edge of edges.values()) {
    if (!backEdgeIds.has(edge.id)) {
      inDegree.set(edge.toNodeId, (inDegree.get(edge.toNodeId) ?? 0) + 1);
    }
  }

  const queue = [...inDegree.entries()]
    .filter(([, deg]) => deg === 0)
    .map(([id]) => id);

  const order: number[] = [];
  while (queue.length > 0) {
    const id = queue.shift()!;
    order.push(id);
    for (const edge of outEdges.get(id) ?? []) {
      if (backEdgeIds.has(edge.id)) continue;
      const remaining = (inDegree.get(edge.toNodeId) ?? 0) - 1;
      inDegree.set(edge.toNodeId, remaining);
      if (remaining === 0) queue.push(edge.toNodeId);
    }
  }

  return order;
}

// ─── Phase 3: assign one output buffer slot per non-sink node ─────────────────
//
// Example:  A → B → D
//                ↘
//                 C
//
//   A  gets slot 0  (has forward out-edges)
//   B  gets slot 1  (has forward out-edges; C and D both read slot 1 — fan-out free)
//   C  no slot      (sink: writes straight to worklet output)
//   D  no slot      (sink: writes straight to worklet output)
//
// If B also had a back-edge going out (cycle), slot 1 would appear in
// feedbackBufferIndices so the worklet saves it at the end of each frame.

interface BufferMap {
  nodeOutputBuf: Map<number, number>; // nodeId → slot index
  bufferCount: number;
  feedbackBufferIndices: number[];    // slots that need a prev-frame copy
}

function assignOutputBuffers(
  executionOrder: number[],
  edges: GraphInput['edges'],
  backEdgeIds: Set<number>,
): BufferMap {
  const outEdgesByNode = buildEdgesByFromFlat(edges);

  const nodeOutputBuf         = new Map<number, number>();
  const feedbackBufferIndices: number[] = [];
  let bufferCount = 0;

  for (const nodeId of executionOrder) {
    const nodeOutEdges  = outEdgesByNode.get(nodeId) ?? [];
    const hasForwardOut = nodeOutEdges.some((e) => !backEdgeIds.has(e.id));
    const hasBackOut    = nodeOutEdges.some((e) =>  backEdgeIds.has(e.id));

    if (hasForwardOut || hasBackOut) {
      nodeOutputBuf.set(nodeId, bufferCount);
      if (hasBackOut) feedbackBufferIndices.push(bufferCount);
      bufferCount++;
    }
  }

  return { nodeOutputBuf, bufferCount, feedbackBufferIndices };
}

// ─── Phase 4: build per-node compiled descriptors ────────────────────────────

function buildNodes(
  executionOrder: number[],
  nodes: GraphInput['nodes'],
  edges: GraphInput['edges'],
  backEdgeIds: Set<number>,
  bufferMap: BufferMap,
): CompiledNode[] {
  const inEdgesByNode  = buildEdgesByTo(edges);
  const outEdgesByNode = buildEdgesByFromFlat(edges);
  const { nodeOutputBuf } = bufferMap;

  return executionOrder.map((nodeId): CompiledNode => {
    const node     = nodes.get(nodeId)!;
    const incoming = inEdgesByNode.get(nodeId)  ?? [];
    const outgoing = outEdgesByNode.get(nodeId) ?? [];

    const hasForwardIn  = incoming.some((e) => !backEdgeIds.has(e.id));
    const hasForwardOut = outgoing.some((e) => !backEdgeIds.has(e.id));

    // Source node: no forward predecessors → read from the worklet's raw audio input.
    const rawSource: NodeInputSource = { kind: 'raw' };

    // Each incoming edge maps to either a current-frame buffer or a feedback buffer.
    const edgeSources: NodeInputSource[] = incoming.map((edge) =>
      backEdgeIds.has(edge.id)
        ? { kind: 'feedback', bufferIndex: nodeOutputBuf.get(edge.fromNodeId)! }
        : { kind: 'buffer',   bufferIndex: nodeOutputBuf.get(edge.fromNodeId)! },
    );

    const inputs: NodeInputSource[] = [
      ...(hasForwardIn ? [] : [rawSource]),
      ...edgeSources,
    ];

    return {
      nodeId,
      transformId:       node.transformId,
      params:            Object.values(node.params),
      inputs,
      outputBufferIndex: hasForwardOut ? nodeOutputBuf.get(nodeId)! : -1,
    };
  });
}

// ─── Edge-map helpers ─────────────────────────────────────────────────────────

/** fromNodeId → edges; every node is pre-seeded (safe for DFS). */
function buildEdgesByFrom(
  nodes: GraphInput['nodes'],
  edges: GraphInput['edges'],
): Map<number, GraphInputEdge[]> {
  const map = new Map<number, GraphInputEdge[]>(
    [...nodes.keys()].map((id) => [id, []]),
  );
  for (const edge of edges.values()) map.get(edge.fromNodeId)?.push(edge);
  return map;
}

/** fromNodeId → edges; only nodes with outgoing edges are present. */
function buildEdgesByFromFlat(edges: GraphInput['edges']): Map<number, GraphInputEdge[]> {
  const map = new Map<number, GraphInputEdge[]>();
  for (const edge of edges.values()) {
    const list = map.get(edge.fromNodeId) ?? [];
    list.push(edge);
    map.set(edge.fromNodeId, list);
  }
  return map;
}

/** toNodeId → edges. */
function buildEdgesByTo(edges: GraphInput['edges']): Map<number, GraphInputEdge[]> {
  const map = new Map<number, GraphInputEdge[]>();
  for (const edge of edges.values()) {
    const list = map.get(edge.toNodeId) ?? [];
    list.push(edge);
    map.set(edge.toNodeId, list);
  }
  return map;
}
