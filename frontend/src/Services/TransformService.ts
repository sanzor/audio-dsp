import type { TransformDefinition } from "@/domain/Transform/TransformDefinition";
import type { TransformPort } from "@/domain/Transform/TransformPort";
import type { TransformParam } from "@/domain/Transform/TransformParam";
import type { CompositeGraphDefinition } from "@/domain/Transform/CompositeGraphDefinition";
import { http, API_BASE_URL, projectApiPath } from "@/Services/http";
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

// Bucket 2 — save. As of the transform-draft API reshape (see
// agents/decisions/0011-transform-draft-api-reshape.md), there is no
// frontend-supplied WASM package anymore — save always compiles
// source_code synchronously server-side and rejects the whole request if
// it doesn't build.
//
// Discriminated union mirroring the backend's untagged SaveDraftParams
// (transform_drafts/dto/requests.rs) exactly — no `kind` field; the server
// picks the variant purely from which field is present.
export type SaveDraftParams =
  | { source_code: string }
  | { graph_definition: CompositeGraphDefinition };

// Bucket 3 — publish. Mirrors the backend's tagged PublishDraftParams
// (transform_drafts/dto/requests.rs) exactly — wire format is
// `{"kind": "primitive"}` or `{"kind": "composite"}`.
export type PublishDraftParams = { kind: "primitive" } | { kind: "composite" };

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

// ─── metadata_json parsing ────────────────────────────────────────────────────
//
// Neither the draft-bucket DTOs (TransformDraftDto) nor the published DTO
// (TransformDto) carry structured `ports`/`params`/`graph_definition` fields
// anymore — everything a transform's authored/introspected shape needs is
// embedded as a JSON string in `metadata_json` (`{name, description, ports,
// params}`, plus `graph` for a composite). This mirrors
// backend/api/src/transform_drafts/processor/transform_metadata.rs's
// PortMetadataJson/ParamMetadataJson exactly on the wire (`order` not
// `port_order`/`param_order`, `default`/`min`/`max` not `default_value`/
// `min_value`/`max_value`, no id field at all — ports/params are derived
// on the fly at compile/validate time, never persisted with an id).
//
// TransformPort/TransformParam (domain/Transform/) intentionally keep their
// existing port_id/port_order/param_id/param_order/default_value/min_value/
// max_value shape unchanged here, since editor-agent's surface
// (GraphCompiler.ts, canvas-panel.tsx, node-details-modal.tsx,
// transform-details-modal.tsx) already depends on exactly that shape and
// this pass is Creator-surface-only. This function is the one place that
// bridges old shape / new wire format: port_id/param_id are synthesized as
// a stable-per-response array index. This is no less stable than what
// existed before — CompositeGraphDefinition.ts's own doc comment already
// documents port_id as "reassigned on every republish," so a consumer
// matching on it was never guaranteed cross-publish stability, only
// stability within one fetched snapshot, which this preserves. See this
// file's note in the final report for the cross-surface follow-up this
// implies (a real, persisted port_id has no HTTP path back to the frontend
// anymore on either controller).
interface PortMetadataJson {
  name: string;
  direction: "input" | "output";
  order: number;
  description?: string | null;
  kind: "program" | "sidechain";
  cardinality: "single" | "many";
}

interface ParamMetadataJson {
  name: string;
  order: number;
  default: number;
  min?: number | null;
  max?: number | null;
  description?: string | null;
}

interface TransformMetadataJson {
  name?: string;
  description?: string | null;
  ports?: PortMetadataJson[];
  params?: ParamMetadataJson[];
  graph?: CompositeGraphDefinition;
}

function parseTransformMetadata(metadataJson: string | null | undefined): {
  ports: TransformPort[];
  params: TransformParam[];
  graph_definition?: CompositeGraphDefinition;
} {
  if (!metadataJson) return { ports: [], params: [] };
  try {
    const parsed = JSON.parse(metadataJson) as TransformMetadataJson;
    const ports: TransformPort[] = (parsed.ports ?? []).map((p, i) => ({
      port_id: i,
      name: p.name,
      direction: p.direction,
      port_order: p.order,
      description: p.description ?? undefined,
      kind: p.kind,
      cardinality: p.cardinality,
    }));
    const params: TransformParam[] = (parsed.params ?? []).map((p, i) => ({
      param_id: i,
      name: p.name,
      param_order: p.order,
      default_value: p.default,
      min_value: p.min ?? undefined,
      max_value: p.max ?? undefined,
      description: p.description ?? undefined,
    }));
    return { ports, params, graph_definition: parsed.graph };
  } catch {
    return { ports: [], params: [] };
  }
}

// Shape actually returned by GET /transforms/{id} and POST /transforms/resolve
// (transforms_controller.rs's TransformDto) — no ports/params/graph_definition/
// is_validated at the top level; ports/params/graph derive from metadata_json.
// `published` isn't present either, pre-existing and unrelated to this reshape
// (see final report) — left as-is (always undefined at runtime), not
// introduced or fixed here.
type TransformDefinitionResponse = Omit<TransformDefinition, "ports" | "params" | "graph_definition"> & {
  metadata_json?: string | null;
};

function normalizeTransformDefinition(response: TransformDefinitionResponse): TransformDefinition {
  const { ports, params, graph_definition } = parseTransformMetadata(response.metadata_json);
  return { ...response, ports, params, graph_definition };
}

export async function apiGetTransformSummaries(offset = 0, limit = 20): Promise<TransformPage> {
  const response = await http.get<{ transforms: TransformSummary[] }>(projectApiPath("/transforms"));

  return {
    transforms: response.transforms.slice(offset, offset + limit),
    total: response.transforms.length,
  };
}

export async function apiGetTransformDefinition(transform_id: number): Promise<TransformDefinition> {
  const response = await http.get<TransformDefinitionResponse>(`/transforms/${transform_id}`);
  return normalizeTransformDefinition(response);
}

export async function apiResolveTransformDefinitions(transform_ids: number[]): Promise<TransformDefinition[]> {
  const response = await http.post<TransformDefinitionsResponse, TransformIdsRequest>(
    `/transforms/resolve`,
    { ids: transform_ids }
  );
  return response.transforms.map(normalizeTransformDefinition);
}

// A transform's in-progress (bucket 2) draft state — mirrors
// backend/api/src/transform_drafts/dto/responses.rs's TransformDraftDto.
// Returned by create/save; nothing currently reads structured
// ports/params/graph off these responses (Save's mutation callers only use
// onSuccess to invalidate the query cache and refetch the full definition
// separately), so this intentionally isn't normalized into TransformDefinition.
export interface TransformDraftDto {
  transform_id: number;
  source_code: string | null;
  metadata_json: string | null;
  name: string | null;
  description: string | null;
  kind: "primitive" | "composite";
  has_binary: boolean;
}

export async function apiCreateTransform(params: CreateTransformParams): Promise<TransformDraftDto> {
  return http.post<TransformDraftDto, CreateTransformParams>(`/draft_transforms`, params);
}

// Draft deletion — only ever allowed server-side for a transform that's
// never been published (409 otherwise). See
// agents/decisions/0002-transform-draft-lifecycle-decisions.md.
export async function apiDeleteTransform(transform_id: number): Promise<void> {
  await http.delete<void>(`/draft_transforms/${transform_id}`);
}

// Bucket 2 — save. Compiles source_code synchronously for the primitive
// variant; the composite variant saves the working graph structurally,
// unconditionally, with no validation (validation is the separate explicit
// apiValidateCompositeGraph action below). Either way, the whole save is
// rejected (nothing persisted) if it doesn't compile/isn't well-formed.
export async function apiSaveTransform(
  transform_id: number,
  params: SaveDraftParams
): Promise<TransformDraftDto> {
  return http.put<TransformDraftDto, SaveDraftParams>(
    `/draft_transforms/${transform_id}/save`,
    params
  );
}

// Standalone compile check — doesn't save. 200 if `source_code` compiles
// cleanly; rejects with the compiler diagnostics text otherwise. Backend
// returns an empty 200 body (not 204) on success, so this bypasses
// Services/http.ts's shared `request` helper (which would try to JSON-parse
// that empty body) rather than changing shared infra for one call site —
// same manual-fetch pattern apiGetTransformBinary below already uses.
export async function apiValidateTransformSourceCode(transform_id: number, source_code: string): Promise<void> {
  const token = useAuthStore.getState().token ?? undefined;
  const activeProjectId = useProjectStore.getState().activeProject?.project_id;
  const response = await fetch(`${API_BASE_URL}/draft_transforms/${transform_id}/validate-source`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
      ...(activeProjectId != null ? { "X-Project-Id": String(activeProjectId) } : {}),
    },
    body: JSON.stringify({ source_code }),
  });
  if (!response.ok) {
    const message = await response.text();
    throw new Error(message || `Request failed: ${response.status}`);
  }
}

// Standalone composite graph validation — validates `graph_json` (which may
// include unsaved canvas edits, unlike the old is_validated model) without
// saving or persisting anything. Ports here are transient — for display
// only, never written back into TransformDefinition.ports.
export interface ValidateGraphPort {
  name: string;
  direction: "input" | "output";
  order: number;
  description?: string;
  kind: "program" | "sidechain";
  cardinality: "single" | "many";
}

export interface ValidateGraphResponse {
  ports: ValidateGraphPort[];
}

export async function apiValidateCompositeGraph(
  transform_id: number,
  graph_definition: CompositeGraphDefinition
): Promise<ValidateGraphResponse> {
  return http.post<ValidateGraphResponse, { graph_json: string }>(
    `/draft_transforms/${transform_id}/validate-graph`,
    { graph_json: JSON.stringify(graph_definition) }
  );
}

// A published (bucket 3) transform — mirrors
// backend/api/src/transforms/dto/responses.rs's TransformDto. Like
// TransformDraftDto above, nothing currently reads structured fields off a
// publish response (callers only use isPending/isError/error.message), so
// this also isn't normalized into TransformDefinition.
export interface PublishedTransformDto {
  transform_id: number;
  name: string;
  description: string | null;
  icon: string | null;
  kind: "primitive" | "composite";
  source_code: string | null;
  metadata_json: string | null;
  owner_user_id: number;
  created_at: string;
}

// Bucket 3 — publish. Bundles whatever's currently saved into the live
// artifact; never compiles. The server independently re-checks the
// declared `kind` against the draft's actual persisted kind and rejects
// with 400 on mismatch (see PublishDraftParams above).
export async function apiPublishTransform(
  transform_id: number,
  params: PublishDraftParams
): Promise<PublishedTransformDto> {
  return http.post<PublishedTransformDto, PublishDraftParams>(
    `/draft_transforms/${transform_id}/publish`,
    params
  );
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
