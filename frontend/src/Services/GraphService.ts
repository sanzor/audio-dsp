import type { Graph } from "@/domain/Graph/Graph";
import type { Edge } from "@/domain/Graph/Edge";
import type { Node, NodeType } from "@/domain/Graph/Node";
import { http } from "@/Services/http";

// ─── Params & Results ────────────────────────────────────────────────────────

interface ApiGraphNode {
  id: number;
  graphId?: number;
  graph_id?: number;
  transformId?: number | null;
  nodeType?: string;
  position?: { x?: number; y?: number };
  params?: Record<string, number>;
}

interface ApiGraphEdge {
  id: number;
  graphId?: number;
  graph_id?: number;
  fromNodeId?: number;
  toNodeId?: number;
  fromPortId?: number | null;
  toPortId?: number | null;
  to?: string;
}

interface ApiGraphRepr {
  schemaVersion?: number;
  nodes?: ApiGraphNode[];
  edges?: ApiGraphEdge[];
}

interface ApiGraphResult {
  id: number;
  region_id: number;
  name: string;
  version: number;
  created_at: string;
  updated_at: string;
  repr: ApiGraphRepr;
}

export interface GetGraphParams {
  graphId: number;
}

export interface CreateGraphParams {
  name: string;
  region_id: number;
}
export interface CreateGraphResult {
  graph: Graph;
}

export interface EditGraphParams {
  id: number;
  name: string;
}
export interface EditGraphResult {
  graph: Graph;
}

export interface SaveGraphStateParams {
  graphId: number;
  state: string;
}
export interface SaveGraphStateResult {
  graph: Graph;
}

export interface RemoveGraphParams {
  graph_id: number;
}
export interface RemoveGraphResult {
  result: string;
}

export interface CopyGraphParams {
  destinationRegionId: number;
  sourceGraphId: number;
  copyName: string;
}
export interface CopyGraphResult {
  graph: Graph;
}

export interface CreateEdgeParams {
  graphId: number;
  fromNodeId: string;
  toNodeId: string;
  name: string | null;
}
export interface CreateEdgeResult {
  edge: Edge;
}

export interface CreateNodeParams {
  graphId: number;
  fromNodeId: string | null;
  toNodeId: string | null;
  name: string;
}
export interface CreateNodeResult {
  id: string;
  graphId: number;
}

const VALID_NODE_TYPES = new Set<NodeType>(["default", "source", "sink"]);

const mapGraphNode = (node: ApiGraphNode, graphId: number): Node => ({
  id: node.id,
  graphId: node.graphId ?? node.graph_id ?? graphId,
  transformId: node.transformId ?? null,
  position: { x: node.position?.x ?? 0, y: node.position?.y ?? 0 },
  params: node.params ?? {},
  ports: [],
  createdAt: new Date(0),
  nodeType: VALID_NODE_TYPES.has(node.nodeType as NodeType)
    ? (node.nodeType as NodeType)
    : "default",
});

const mapGraphEdge = (edge: ApiGraphEdge, graphId: number): Edge => ({
  id: edge.id,
  graphId: edge.graphId ?? edge.graph_id ?? graphId,
  fromNodeId: edge.fromNodeId ?? 0,
  toNodeId: edge.toNodeId ?? 0,
  fromPortId: edge.fromPortId ?? null,
  toPortId: edge.toPortId ?? null,
  to: edge.to ?? "",
});

const mapGraph = (graph: ApiGraphResult): Graph => ({
  id: graph.id,
  regionId: graph.region_id,
  name: graph.name,
  createdAt: new Date(graph.created_at),
  updatedAt: new Date(graph.updated_at),
  nodes: (graph.repr?.nodes ?? []).map((node) => mapGraphNode(node, graph.id)),
  edges: (graph.repr?.edges ?? []).map((edge) => mapGraphEdge(edge, graph.id)),
});

// ─── API ──────────────────────────────────────────────────────────────────────

export async function apiGetGraph(graphId: number): Promise<Graph> {
  const result = await http.get<ApiGraphResult>(`/graphs/get-graph?graph_id=${graphId}`);
  return mapGraph(result);
}

export async function apiCreateGraph(params: CreateGraphParams): Promise<CreateGraphResult> {
  const response = await http.post<{ graph: ApiGraphResult }, CreateGraphParams>("/graphs/create", params);
  return { graph: mapGraph(response.graph) };
}

export async function apiUpdateGraph(params: EditGraphParams): Promise<EditGraphResult> {
  const response = await http.patch<{ graph: ApiGraphResult }, EditGraphParams>("/graphs/edit", params);
  return { graph: mapGraph(response.graph) };
}

export async function apiSaveGraphState(params: SaveGraphStateParams): Promise<SaveGraphStateResult> {
  const response = await http.put<{ graph: ApiGraphResult }, { graphId: number; state: string }>(
    "/graphs/save-state",
    { graphId: params.graphId, state: params.state },
  );
  return { graph: mapGraph(response.graph) };
}

export async function apiRemoveGraph(params: RemoveGraphParams): Promise<void> {
  await http.delete(`/graphs/remove?graph_id=${params.graph_id}`);
}

export async function apiCopyGraph(params: CopyGraphParams): Promise<CopyGraphResult> {
  const response = await http.post<{ graph: ApiGraphResult }, CopyGraphParams>("/graphs/copy", params);
  return { graph: mapGraph(response.graph) };
}
