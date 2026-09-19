import type { TransformDefinition } from "@/domain/Transform/TransformDefinition";
import type { TransformPort } from "@/domain/Transform/TransformPort";
import type { TransformParam } from "@/domain/Transform/TransformParam";
import type { CompositeGraphDefinition } from "@/domain/Transform/CompositeGraphDefinition";
import { http, API_BASE_URL, projectApiPath } from "@/Services/http";
import { useAuthStore } from "@/Stores/authStore";
import { useProjectStore } from "@/Stores/projectStore";
import type { TransformSummary } from "@/domain/Transform/TransformSummary";

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
// Neither the draft-bucket DTOs (Services/transform-drafts/TransformDraftsService.ts's
// TransformDraftDefinition) nor the published DTO (TransformDto) carry
// structured `ports`/`params`/`graph_definition` fields anymore — everything
// a transform's authored/introspected shape needs is embedded as a JSON
// string in `metadata_json` (`{name, description, ports, params}`, plus
// `graph` for a composite). This mirrors
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

// "The store" — transforms that have actually been published at least once
// (GET /v1/workspaces/{id}/transforms/published), as opposed to every
// transform the caller can see (drafts included) from apiGetTransformSummaries
// above. This is what the composite canvas offers as draggable leaves — an
// unpublished draft has no resolvable artifact yet.
export async function apiGetPublishedTransformSummaries(): Promise<TransformSummary[]> {
  const response = await http.get<{ transforms: TransformSummary[] }>(projectApiPath("/transforms/published"));
  return response.transforms;
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
