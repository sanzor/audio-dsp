import type { Transform } from "@/domain/Transform/Transform";
import { http } from "@/Services/http";

// ─── Params ──────────────────────────────────────────────────────────────────

export interface CreateTransformParams {
  name: string;
  description?: string;
  icon?: string;
}

export interface UpdateTransformParams {
  name: string;
  description?: string;
  icon?: string;
}

export interface AddPortParams {
  name: string;
  direction: "input" | "output";
  port_order: number;
  description?: string;
}

// ─── API ─────────────────────────────────────────────────────────────────────

export interface TransformPage {
  transforms: Transform[];
  total: number;
}

export async function apiGetTransforms(offset = 0, limit = 20): Promise<TransformPage> {
  return http.get<TransformPage>(`/transforms/get-all?offset=${offset}&limit=${limit}`);
}

export async function apiGetTransformById(transform_id: number): Promise<Transform> {
  return http.get<Transform>(`/transforms/get-by-id?transform_id=${transform_id}`);
}

export async function apiCreateTransform(params: CreateTransformParams): Promise<Transform> {
  return http.post<Transform, CreateTransformParams>(`/transforms/create`, params);
}

export async function apiUpdateTransform(
  transform_id: number,
  params: UpdateTransformParams
): Promise<Transform> {
  return http.put<Transform, UpdateTransformParams>(
    `/transforms/update?transform_id=${transform_id}`,
    params
  );
}

export async function apiDeleteTransform(transform_id: number): Promise<void> {
  await http.delete<void>(`/transforms/delete?transform_id=${transform_id}`);
}

export async function apiAddPort(
  transform_id: number,
  params: AddPortParams
): Promise<Transform> {
  return http.post<Transform, AddPortParams>(
    `/transforms/add-port?transform_id=${transform_id}`,
    params
  );
}

export async function apiDeletePort(port_id: number): Promise<void> {
  await http.delete<void>(`/transforms/delete-port?port_id=${port_id}`);
}

// Fetches the pre-compiled .wasm binary for a transform from the backend.
// The binary is stored in S3 and served via the backend — the frontend never compiles.
export async function apiGetTransformWasm(transform_id: number): Promise<Uint8Array> {
  const response = await fetch(`/transforms/wasm?transform_id=${transform_id}`);
  if (!response.ok) throw new Error(`Failed to fetch wasm for transform ${transform_id}`);
  const buffer = await response.arrayBuffer();
  return new Uint8Array(buffer);
}
