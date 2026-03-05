import type { Graph } from "@/Domain/Graph/Graph";
import type { Edge } from "@/Domain/Graph/Edge";

const BASE_URL = import.meta.env.VITE_API_BASE_URL;

// ─── Params & Results ────────────────────────────────────────────────────────

export interface GetGraphParams {
  graphId: string;
}

export interface CreateGraphParams {
  name: string;
  region_id: string;
}
export interface CreateGraphResult {
  graph: Graph;
}

export interface EditGraphParams {
  id: string;
  name: string;
}
export interface EditGraphResult {
  graph: Graph;
}

export interface RemoveGraphParams {
  graph_id: string;
}
export interface RemoveGraphResult {
  result: string;
}

export interface CopyGraphParams {
  destinationRegionId: string;
  sourceGraphId: string;
  copyName: string;
}
export interface CopyGraphResult {
  graph: Graph;
}

export interface CreateEdgeParams {
  graphId: string;
  fromNodeId: string;
  toNodeId: string;
  name: string | null;
}
export interface CreateEdgeResult {
  edge: Edge;
}

export interface CreateNodeParams {
  graphId: string;
  fromNodeId: string | null;
  toNodeId: string | null;
  name: string;
}
export interface CreateNodeResult {
  id: string;
  graphId: string;
}

// ─── API ──────────────────────────────────────────────────────────────────────

export async function apiGetGraph(graphId: string): Promise<Graph> {
  const res = await fetch(`${BASE_URL}/graphs/get-graph?graph_id=${graphId}`, {
    method: 'GET',
    credentials: 'include',
  });
  if (!res.ok) throw new Error('Failed to fetch graph');
  const json = await res.json();
  console.log(json);
  return json.tracks;
}

export async function apiCreateGraph(params: CreateGraphParams): Promise<CreateGraphResult> {
  console.log("a");
  const res = await fetch(`${BASE_URL}/graphs/create`, {
    method: 'POST',
    credentials: 'include',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(params),
  });
  console.log(res.status);
  if (!res.ok) throw new Error(`Failed to create graph: ${res.statusText}`);
  return await res.json();
}

export async function apiUpdateGraph(params: EditGraphParams): Promise<EditGraphResult> {
  console.log("a");
  const res = await fetch(`${BASE_URL}/graphs/edit`, {
    method: 'PATCH',
    credentials: 'include',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(params),
  });
  console.log(res.status);
  if (!res.ok) throw new Error(`Failed to update graph: ${res.statusText}`);
  return await res.json();
}

export async function apiRemoveGraph(params: RemoveGraphParams): Promise<void> {
  const res = await fetch(`${BASE_URL}/graphs/remove?graph_id=${params.graph_id}`, {
    method: 'DELETE',
    credentials: 'include',
  });
  if (!res.ok) throw new Error('Refresh token failed');
}

export async function apiCopyGraph(params: CopyGraphParams): Promise<CopyGraphResult> {
  const res = await fetch(`${BASE_URL}/graphs/copy`, {
    method: 'POST',
    credentials: 'include',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(params),
  });
  if (!res.ok) throw new Error(`Failed to copy graph: ${res.statusText}`);
  return await res.json();
}
