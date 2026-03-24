import type { Transform } from "@/domain/Transform/Transform";
import { http } from "@/Services/http";

const BASE_URL = import.meta.env.VITE_API_BASE_URL;

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
  return http.get<TransformPage>(`${BASE_URL}/transforms/get-all?offset=${offset}&limit=${limit}`);
}

export async function apiGetTransformById(transform_id: number): Promise<Transform> {
  return http.get<Transform>(`${BASE_URL}/transforms/get-by-id?transform_id=${transform_id}`);
}

export async function apiCreateTransform(params: CreateTransformParams): Promise<Transform> {
  return http.post<Transform, CreateTransformParams>(`${BASE_URL}/transforms/create`, params);
}

export async function apiUpdateTransform(
  transform_id: number,
  params: UpdateTransformParams
): Promise<Transform> {
  return http.put<Transform, UpdateTransformParams>(
    `${BASE_URL}/transforms/update?transform_id=${transform_id}`,
    params
  );
}

export async function apiDeleteTransform(transform_id: number): Promise<void> {
  await http.delete<void>(`${BASE_URL}/transforms/delete?transform_id=${transform_id}`);
}

export async function apiAddPort(
  transform_id: number,
  params: AddPortParams
): Promise<Transform> {
  return http.post<Transform, AddPortParams>(
    `${BASE_URL}/transforms/add-port?transform_id=${transform_id}`,
    params
  );
}

export async function apiDeletePort(port_id: number): Promise<void> {
  await http.delete<void>(`${BASE_URL}/transforms/delete-port?port_id=${port_id}`);
}
