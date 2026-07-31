import { useTransformStore } from '@/Stores/TransformStore';
import type { ActiveGraph } from '@/Stores/ActiveGraphState';
import type { TransformPort } from '@/domain/Transform/TransformPort';
import type { CompiledGraph } from './compile-graph/compiledGraph';

// ─── Input types ─────────────────────────────────────────────────────────────

export interface GraphInputNode {
  id: number;
  transformId: number;
  params: Record<string, number>;
  // Count of this transform's declared input ports — determines how many
  // separate buffers its process() call receives (see transform-sdk's
  // `Transform::process(samples: &[&[f32]], ...)` contract: one slice per
  // declared input port, in port_order).
  inputPortCount: number;
}

export interface GraphInputEdge {
  id: number;
  fromNodeId: number;
  toNodeId: number;
  // 0-based ordinal among the target node's declared input ports, ordered by
  // port_order — says which of the node's process() input slices this edge
  // feeds. Multi-input-port transforms (e.g. a sidechain) need this to keep
  // separate signals separate instead of everything summed into one buffer.
  toPortIndex: number;
}

export interface GraphInput {
  nodes: Map<number, GraphInputNode>;
  edges: Map<number, GraphInputEdge>;
}

// ─── Port lookup helpers ──────────────────────────────────────────────────────
// Shared by both GraphInput producers below (compileGraph for Editor graphs,
// composite-canvas.tsx for composite preview) — resolves each caller's own
// port identifier scheme (numeric port_id for Editor edges, port name for
// composite edges) against the node's own port list from useTransformStore.

function inputPortsOf(transformId: number): TransformPort[] {
  return (useTransformStore.getState().definitions.get(transformId)?.ports ?? [])
    .filter((p) => p.direction === 'input')
    .sort((a, b) => a.port_order - b.port_order);
}

// Falls back to 1 (today's single-input-port assumption) when the
// transform's definition hasn't been fetched into the store yet, so a node
// whose ports simply haven't loaded doesn't get starved of input.
export function inputPortCountOf(transformId: number, fallback = 1): number {
  const def = useTransformStore.getState().definitions.get(transformId);
  return def ? inputPortsOf(transformId).length : fallback;
}

export function inputPortIndexById(transformId: number, portId: number): number {
  const idx = inputPortsOf(transformId).findIndex((p) => p.port_id === portId);
  return idx === -1 ? 0 : idx;
}

export function inputPortIndexByName(transformId: number, portName: string): number {
  const idx = inputPortsOf(transformId).findIndex((p) => p.name === portName);
  return idx === -1 ? 0 : idx;
}

// ─── Output types ─────────────────────────────────────────────────────────────

export type NodeInputSource =
  | { kind: 'raw' }
  | { kind: 'buffer';   bufferIndex: number }
  | { kind: 'feedback'; bufferIndex: number };

export interface CompiledNode {
  nodeId: number;
  transformId: number;
  params: number[];             // flattened from Record<string,number>, sent to WASM
  // One bucket per declared input port (index = port ordinal); each bucket is
  // the list of sources summed together to produce that port's buffer.
  inputs: NodeInputSource[][];
  // -1 when the node has neither a forward consumer nor a back-edge out (no
  // buffer needed); otherwise a real slot index that gets written each
  // quantum. NOTE: this is no longer "is this the terminal/audible node" —
  // a node can hold a real buffer index AND also need to reach the worklet
  // output, when it has a back-edge but no forward consumer (see
  // writesToOutput below). Kept in sync with feedbackBufferIndices by
  // assignOutputBuffers().
  outputBufferIndex: number;
  // True iff this node has no forward (non-back-edge) outgoing edge, i.e.
  // nothing downstream consumes it within the graph, so its output must
  // also be summed into the worklet's audible output — independent of
  // whether it separately has hasBackOut and therefore a real
  // outputBufferIndex too. Computed as `!hasForwardOut` in buildNodes().
  writesToOutput: boolean;
}


// ─── Result type ──────────────────────────────────────────────────────────────

export type CompileResult =
  | { ok: true;  graph: CompiledGraph }
  | { ok: false; reason: 'empty_graph' };

// ─── High-level entry point ───────────────────────────────────────────────────
//
// Converts an ActiveGraph into a compiled graph descriptor.
// Returns null if the graph is empty/invalid.
export function compileGraph(
  graph: ActiveGraph,
): CompiledGraph | null {
  const input: GraphInput = {
    nodes: new Map(
      [...graph.nodes.entries()].map(([id, node]) => [
        id,
        { id, transformId: node.transformId, params: node.params, inputPortCount: inputPortCountOf(node.transformId) },
      ]),
    ),
    edges: new Map(
      [...graph.edges.entries()].map(([id, edge]) => {
        const toTransformId = graph.nodes.get(edge.toNodeId)?.transformId;
        return [
          id,
          {
            id,
            fromNodeId: edge.fromNodeId,
            toNodeId: edge.toNodeId,
            toPortIndex: toTransformId != null ? inputPortIndexById(toTransformId, edge.toPortId) : 0,
          },
        ];
      }),
    ),
  };

  const result = process(input);
  if (!result.ok) return null;
  return result.graph;
}

// ─── Entry point ──────────────────────────────────────────────────────────────
//
// Pure function: GraphInput → CompileResult.  No side effects, no I/O.
//
// Phases
// ──────
// 1. findBackEdges       — DFS; marks edges that close a cycle (back-edges).
//                          A back-edge points to an ancestor still on the DFS
//                          stack, meaning "this node feeds audio back to an
//                          earlier node."  We break the cycle at runtime by
//                          using the upstream node's output from the *previous*
//                          audio frame instead of the current one.
//
// 2. topoSort            — Kahn's BFS on the forward-edge DAG (back-edges
//                          excluded).  Produces an order where every node's
//                          upstream dependencies are already processed.
//
// 3. assignOutputBuffers — Gives each non-sink node one numbered buffer slot
//                          (a Float32Array the worklet allocates).  A node
//                          writes its output once into its slot; every
//                          downstream node reads from the same slot (fan-out
//                          is free).  Sink nodes (nothing downstream) write
//                          straight to the worklet output — no slot needed.
//                          Nodes that also have back-edges out are flagged so
//                          the worklet knows to save their slot at frame end.
//
// 4. buildNodes          — Assembles the final CompiledNode list in execution
//                          order, wiring up which slots each node reads from
//                          and which slot (or the raw output) it writes to.

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
    color.set(id, GRAY);
    for (const edge of outEdges.get(id) ?? []) {
      const neighborColor = color.get(edge.toNodeId) ?? WHITE;
      if (neighborColor === GRAY)       backEdgeIds.add(edge.id);
      else if (neighborColor === WHITE) visit(edge.toNodeId);
    }
    color.set(id, BLACK);
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

interface BufferMap {
  nodeOutputBuf: Map<number, number>;
  bufferCount: number;
  feedbackBufferIndices: number[];
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
    const hasBackOut    = outgoing.some((e) =>  backEdgeIds.has(e.id));

    // One bucket per declared input port; each edge's toPortIndex says which
    // bucket it feeds. A bucket with no edges resolves to silence in the
    // worklet, except port 0 on a true source node (no forward incoming
    // edges at all), which falls back to the raw pipeline input — same as
    // every existing single-input-port transform already gets.
    const inputs: NodeInputSource[][] = Array.from({ length: node.inputPortCount }, () => []);
    for (const edge of incoming) {
      const bucket = inputs[edge.toPortIndex];
      if (!bucket) continue; // defensive: stale/out-of-range port index
      bucket.push(
        backEdgeIds.has(edge.id)
          ? { kind: 'feedback', bufferIndex: nodeOutputBuf.get(edge.fromNodeId)! }
          : { kind: 'buffer',   bufferIndex: nodeOutputBuf.get(edge.fromNodeId)! },
      );
    }
    if (!hasForwardIn && inputs.length > 0) {
      inputs[0] = [{ kind: 'raw' }, ...inputs[0]];
    }

    return {
      nodeId,
      transformId:       node.transformId,
      params:            Object.values(node.params),
      inputs,
      outputBufferIndex: (hasForwardOut || hasBackOut) ? nodeOutputBuf.get(nodeId)! : -1,
      writesToOutput: !hasForwardOut,
    };
  });
}

// ─── Edge-map helpers ─────────────────────────────────────────────────────────

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

function buildEdgesByFromFlat(edges: GraphInput['edges']): Map<number, GraphInputEdge[]> {
  const map = new Map<number, GraphInputEdge[]>();
  for (const edge of edges.values()) {
    const list = map.get(edge.fromNodeId) ?? [];
    list.push(edge);
    map.set(edge.fromNodeId, list);
  }
  return map;
}

function buildEdgesByTo(edges: GraphInput['edges']): Map<number, GraphInputEdge[]> {
  const map = new Map<number, GraphInputEdge[]>();
  for (const edge of edges.values()) {
    const list = map.get(edge.toNodeId) ?? [];
    list.push(edge);
    map.set(edge.toNodeId, list);
  }
  return map;
}
