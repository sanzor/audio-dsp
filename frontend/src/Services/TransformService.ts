import type { TransformDefinition } from "@/domain/Transform/TransformDefinition";
import type { CompositeGraphDefinition } from "@/domain/Transform/CompositeGraphDefinition";
import { http, API_BASE_URL } from "@/Services/http";
import { useAuthStore } from "@/Stores/authStore";
import { useProjectStore } from "@/Stores/projectStore";
import type { TransformSummary } from "@/domain/Transform/TransformSummary";

// ─── Params ──────────────────────────────────────────────────────────────────

export interface CreateTransformParams {
  name: string;
  description?: string;
  icon?: string;
  kind: "primitive" | "composite";
}

export interface SaveTransformParams {
  source_code: string;
  // A resource_id from a successful compile ticket, if attaching that
  // build's binary/metadata to this save. Omit to save source only.
  resource_id?: number;
}

// ─── API ─────────────────────────────────────────────────────────────────────

export interface TransformPage {
  transforms: TransformSummary[];
  total: number;
}

export interface TransformDefinitionsResponse {
  transforms: TransformDefinition[];
}

export interface TransformIdsRequest {
  ids: number[];
}

export interface TransformBinaryEnvelope {
  transform_id: number;
  wasm_base64: string;
}

export interface TransformBinariesResponse {
  binaries: TransformBinaryEnvelope[];
}

// Exported for reuse by the creator surface's "Try it" preview flow, which
// decodes a compile ticket/resource's wasm_base64 the same way this file
// already decodes published TransformBinaryDto payloads — see
// agents/decisions/0003-transform-preview-flow.md.
export function decodeBase64Binary(encoded: string): Uint8Array {
  const decoded = atob(encoded);
  const bytes = new Uint8Array(decoded.length);
  for (let i = 0; i < decoded.length; i += 1) {
    bytes[i] = decoded.charCodeAt(i);
  }
  return bytes;
}

export function mapTransformBinariesResponse(response: TransformBinariesResponse): Map<number, Uint8Array> {
  return new Map(
    response.binaries.map((binary) => [
      binary.transform_id,
      decodeBase64Binary(binary.wasm_base64),
    ])
  );
}

export async function apiGetTransformSummaries(offset = 0, limit = 20): Promise<TransformPage> {
  return http.get<TransformPage>(`/transforms?offset=${offset}&limit=${limit}`);
}

export async function apiGetTransformDefinition(transform_id: number): Promise<TransformDefinition> {
  return http.get<TransformDefinition>(`/transforms/${transform_id}`);
}

export async function apiResolveTransformDefinitions(transform_ids: number[]): Promise<TransformDefinition[]> {
  const response = await http.post<TransformDefinitionsResponse, TransformIdsRequest>(
    `/transforms/resolve`,
    { ids: transform_ids }
  );
  return response.transforms;
}

export async function apiCreateTransform(params: CreateTransformParams): Promise<TransformDefinition> {
  return http.post<TransformDefinition, CreateTransformParams>(`/transforms`, params);
}

export async function apiDeleteTransform(transform_id: number): Promise<void> {
  await http.delete<void>(`/transforms/${transform_id}`);
}

// Bucket 2 — save. Always overwrites source_code; optionally attaches a
// compiled resource's binary/metadata. Independent of compiling — never
// blocked by or waiting on an in-flight compile ticket.
export async function apiSaveTransform(
  transform_id: number,
  params: SaveTransformParams
): Promise<TransformDefinition> {
  return http.put<TransformDefinition, SaveTransformParams>(
    `/transforms/${transform_id}/save`,
    params
  );
}

// Bucket 3 — publish. Bundles whatever's currently saved into the live
// artifact. Does not compile — fails if nothing saved has a binary yet.
export async function apiPublishTransform(transform_id: number): Promise<TransformDefinition> {
  return http.post<TransformDefinition, undefined>(`/transforms/${transform_id}/publish`, undefined);
}

// Composite counterpart to apiSaveTransform — writes the working wiring
// graph instead of source_code; validates and derives the composite's own
// exposed ports server-side (see composite_validator::validate_composite_graph).
export interface SaveCompositeGraphParams {
  graph_definition: CompositeGraphDefinition;
}

export async function apiSaveCompositeTransform(
  transform_id: number,
  graph_definition: CompositeGraphDefinition
): Promise<TransformDefinition> {
  return http.put<TransformDefinition, SaveCompositeGraphParams>(
    `/transforms/${transform_id}/save-composite`,
    { graph_definition }
  );
}

// New explicit validate action for a composite draft (see
// agents/decisions/0007-composite-draft-validation-gate.md). Runs the same
// composite_validator::validate_composite_graph check Save used to run,
// against the currently-persisted graph_definition. On success, the
// returned definition has its derived ports and is_validated: true; on
// failure (rejected promise, message from http.ts's response.text()) ports/
// is_validated are left untouched server-side.
export async function apiValidateCompositeTransform(transform_id: number): Promise<TransformDefinition> {
  return http.post<TransformDefinition, undefined>(`/transforms/${transform_id}/validate`, undefined);
}

// Advisory pre-publish check (multi-input named ports republish warning).
// Diffs the port shape about to be published (bucket 2, saved state)
// against what's currently live (bucket 3, transform_ports) — count/kind/
// cardinality per port, not name. Never mutates anything and is never
// called implicitly by apiPublishTransform; the creator's Publish flow
// polls it first and shows a non-blocking confirm dialog when `changed` is
// true. `changed` is always false for a transform that's never been
// published (nothing to compare against yet).
export interface PortShapeSummary {
  name: string;
  direction: "input" | "output";
  kind: "program" | "sidechain";
  cardinality: "single" | "many";
}

export interface PublishPortShapeDiff {
  changed: boolean;
  current: PortShapeSummary[];
  incoming: PortShapeSummary[];
}

export async function apiGetPublishPortShapeDiff(transform_id: number): Promise<PublishPortShapeDiff> {
  return http.get<PublishPortShapeDiff>(`/transforms/${transform_id}/publish/port-diff`);
}

// Fetches the pre-compiled .wasm binary for a transform from the backend.
// The backend reads the committed binary bytes from persisted storage; the frontend never compiles.
export async function apiGetTransformBinary(transform_id: number): Promise<Uint8Array> {
  const token = useAuthStore.getState().token ?? undefined;
  const activeProjectId = useProjectStore.getState().activeProject?.project_id;
  const response = await fetch(`${API_BASE_URL}/transforms/${transform_id}/binary`, {
    headers: {
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
      ...(activeProjectId != null ? { "X-Project-Id": String(activeProjectId) } : {}),
    },
  });
  if (!response.ok) throw new Error(`Failed to fetch wasm for transform ${transform_id}`);
  const buffer = await response.arrayBuffer();
  return new Uint8Array(buffer);
}

export async function apiFetchTransformBinaries(request: TransformIdsRequest): Promise<TransformBinariesResponse> {
  return http.post<TransformBinariesResponse, TransformIdsRequest>(
    `/transforms/binaries`,
    request
  );
}

export async function apiGetTransformBinaries(transform_ids: number[]): Promise<Map<number, Uint8Array>> {
  const response = await apiFetchTransformBinaries({ ids: transform_ids });
  return mapTransformBinariesResponse(response);
}
