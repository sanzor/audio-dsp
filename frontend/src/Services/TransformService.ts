import type { Transform } from "@/domain/Transform/Transform";

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
  const res = await fetch(
    `${BASE_URL}/transforms/get-all?offset=${offset}&limit=${limit}`,
    { method: "GET", credentials: "include" }
  );
  if (!res.ok) throw new Error("Failed to fetch transforms");
  return res.json();
}

export async function apiGetTransformById(transform_id: number): Promise<Transform> {
  const res = await fetch(`${BASE_URL}/transforms/get-by-id?transform_id=${transform_id}`, {
    method: "GET",
    credentials: "include",
  });
  if (!res.ok) throw new Error("Failed to fetch transform");
  return res.json();
}

export async function apiCreateTransform(params: CreateTransformParams): Promise<Transform> {
  const res = await fetch(`${BASE_URL}/transforms/create`, {
    method: "POST",
    credentials: "include",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(params),
  });
  if (!res.ok) throw new Error("Failed to create transform");
  return res.json();
}

export async function apiUpdateTransform(
  transform_id: number,
  params: UpdateTransformParams
): Promise<Transform> {
  const res = await fetch(`${BASE_URL}/transforms/update?transform_id=${transform_id}`, {
    method: "PUT",
    credentials: "include",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(params),
  });
  if (!res.ok) throw new Error("Failed to update transform");
  return res.json();
}

export async function apiDeleteTransform(transform_id: number): Promise<void> {
  const res = await fetch(`${BASE_URL}/transforms/delete?transform_id=${transform_id}`, {
    method: "DELETE",
    credentials: "include",
  });
  if (!res.ok) throw new Error("Failed to delete transform");
}

export async function apiAddPort(
  transform_id: number,
  params: AddPortParams
): Promise<Transform> {
  const res = await fetch(`${BASE_URL}/transforms/add-port?transform_id=${transform_id}`, {
    method: "POST",
    credentials: "include",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(params),
  });
  if (!res.ok) throw new Error("Failed to add port");
  return res.json();
}

export async function apiDeletePort(port_id: number): Promise<void> {
  const res = await fetch(`${BASE_URL}/transforms/delete-port?port_id=${port_id}`, {
    method: "DELETE",
    credentials: "include",
  });
  if (!res.ok) throw new Error("Failed to delete port");
}
