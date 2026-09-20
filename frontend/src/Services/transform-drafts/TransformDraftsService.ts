import type { CompositeGraphDefinition } from "@/domain/Transform/CompositeGraphDefinition";
import type { TransformDraftDefinition } from "@/domain/transform-drafts/TransformDraftDefinition";
import { http, API_BASE_URL } from "@/Services/http";
import { useAuthStore } from "@/Stores/authStore";
import { useProjectStore } from "@/Stores/projectStore";

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
// picks the variant purely from which field is present. `name`/`description`
// are optional on both variants — folded into Save rather than living behind
// a separate endpoint (see
// agents/decisions/0012-draft-name-description-editable.md). Omit the keys
// entirely for a save that isn't touching metadata; the backend's `None`
// (absent key or explicit `null`) leaves the existing value untouched.
export type SaveDraftParams =
  | { source_code: string; name?: string; description?: string | null }
  | { graph_definition: CompositeGraphDefinition; name?: string; description?: string | null };

// Bucket 2 — publish (route lives under /draft_transforms/*, despite the
// name). Mirrors the backend's tagged PublishDraftParams
// (transform_drafts/dto/requests.rs) exactly — wire format is
// `{"kind": "primitive"}` or `{"kind": "composite"}`.
export type PublishDraftParams = { kind: "primitive" } | { kind: "composite" };

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

// Response to a publish call (POST /draft_transforms/{id}/publish) — mirrors
// backend/api/src/transforms/dto/responses.rs's TransformDto shape on the
// wire, but intentionally given its own name/definition here rather than
// importing anything from the bucket-3 domain (domain/Transform/), to avoid
// introducing cross-bucket type coupling for a bucket-2 action. Like
// TransformDraftDefinition above, nothing currently reads structured fields
// off a publish response (callers only use isPending/isError/error.message).
export interface PublishDraftResponse {
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

// ─── API ─────────────────────────────────────────────────────────────────────

export async function apiCreateTransform(params: CreateTransformParams): Promise<TransformDraftDefinition> {
  return http.post<TransformDraftDefinition, CreateTransformParams>(`/draft_transforms`, params);
}

// Bucket 2 — read a single draft (name/description/source/metadata as
// currently persisted on `transform_draft`), separate from the bucket-3
// published `TransformDefinition` (hooks/transforms/queries.ts). Used by
// the Creator's properties panel to source the editable name/description
// fields, since bucket-3's `name`/`description` only ever reflect what was
// last published.
export async function apiGetTransformDraft(transform_id: number): Promise<TransformDraftDefinition> {
  return http.get<TransformDraftDefinition>(`/draft_transforms/${transform_id}`);
}

// Batched draft fetch — mirrors Services/TransformService.ts's
// apiResolveTransformDefinitions (bucket 3) shape/pattern exactly, but for
// bucket 2. Used by the left transforms-sidebar to resolve each listed
// transform's current draft name/description client-side, so a Save-edited
// rename shows up there without any backend sync (see
// agents/decisions/0012-draft-name-description-editable.md's addendum).
// Backend response shape is `{drafts: [...]}` (TransformDraftsResponse),
// not `{transforms: [...]}` like the bucket-3 equivalent.
export interface TransformDraftsResponse {
  drafts: TransformDraftDefinition[];
}

export async function apiResolveTransformDrafts(transform_ids: number[]): Promise<TransformDraftDefinition[]> {
  const response = await http.post<TransformDraftsResponse, { ids: number[] }>(
    `/draft_transforms/resolve`,
    { ids: transform_ids }
  );
  return response.drafts;
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
): Promise<TransformDraftDefinition> {
  return http.put<TransformDraftDefinition, SaveDraftParams>(
    `/draft_transforms/${transform_id}/save`,
    params
  );
}

// Standalone compile check — doesn't save. 200 if `source_code` compiles
// cleanly; rejects with the compiler diagnostics text otherwise. Backend
// returns an empty 200 body (not 204) on success, so this bypasses
// Services/http.ts's shared `request` helper (which would try to JSON-parse
// that empty body) rather than changing shared infra for one call site —
// same manual-fetch pattern Services/TransformService.ts's
// apiGetTransformBinary uses for its bucket-3 equivalent.
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

/** Fetches the binary already attached by a successful primitive-draft Save.
 * This read never compiles, saves, or publishes the transform. */
export async function apiGetPrimitiveDraftBinary(transform_id: number): Promise<Uint8Array> {
  const token = useAuthStore.getState().token ?? undefined;
  const activeProjectId = useProjectStore.getState().activeProject?.project_id;
  const response = await fetch(`${API_BASE_URL}/draft_transforms/${transform_id}/binary`, {
    headers: {
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
      ...(activeProjectId != null ? { "X-Project-Id": String(activeProjectId) } : {}),
    },
  });
  if (!response.ok) {
    const message = await response.text();
    throw new Error(message || `Could not fetch saved draft binary: ${response.status}`);
  }
  return new Uint8Array(await response.arrayBuffer());
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

// Bucket 2 — publish (route lives under /draft_transforms/*, despite
// publishing being what makes a transform live in bucket 3). Bundles
// whatever's currently saved into the live artifact; never compiles. The
// server independently re-checks the declared `kind` against the draft's
// actual persisted kind and rejects with 400 on mismatch (see
// PublishDraftParams above).
export async function apiPublishTransform(
  transform_id: number,
  params: PublishDraftParams
): Promise<PublishDraftResponse> {
  return http.post<PublishDraftResponse, PublishDraftParams>(
    `/draft_transforms/${transform_id}/publish`,
    params
  );
}
