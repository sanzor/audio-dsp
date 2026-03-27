import type { Graph } from "@/domain/Graph/Graph";
import type { Edge } from "@/domain/Graph/Edge";
import { http } from "@/Services/http";

// ─── Params & Results ────────────────────────────────────────────────────────

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

// ─── API ──────────────────────────────────────────────────────────────────────

export function apiGetGraph(graphId: number): Promise<Graph> {
  return http.get(`/graphs/get-graph?graph_id=${graphId}`);
}

export function apiCreateGraph(params: CreateGraphParams): Promise<CreateGraphResult> {
  return http.post("/graphs/create", params);
}

export function apiUpdateGraph(params: EditGraphParams): Promise<EditGraphResult> {
  return http.patch("/graphs/edit", params);
}

export function apiRemoveGraph(params: RemoveGraphParams): Promise<void> {
  return http.delete(`/graphs/remove?graph_id=${params.graph_id}`);
}

export function apiCopyGraph(params: CopyGraphParams): Promise<CopyGraphResult> {
  return http.post("/graphs/copy", params);
}
