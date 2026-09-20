import { useMutation, useQueryClient } from "@tanstack/react-query";
import {
  apiCreateTransform,
  apiSaveTransform,
  apiValidateTransformSourceCode,
  apiValidateCompositeGraph,
  apiPublishTransform,
  apiDeleteTransform,
  type CreateTransformParams,
  type SaveDraftParams,
  type PublishDraftParams,
} from "@/Services/transform-drafts/TransformDraftsService";
import type { CompositeGraphDefinition } from "@/domain/Transform/CompositeGraphDefinition";
import { useTransformDraftStore } from "@/Stores/transform-drafts/TransformDraftStore";
import { QUERY_KEYS } from "@/constants/queryKeys";

export function useCreateTransform() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (params: CreateTransformParams) => apiCreateTransform(params),
    onSuccess: (draft) => {
      useTransformDraftStore.getState().upsertDraft(draft);
      qc.invalidateQueries({ queryKey: QUERY_KEYS.transforms.all() });
    },
  });
}

// Bucket 2 — save. Takes a discriminated union: `{source_code}` compiles
// synchronously server-side and rejects the whole save if it doesn't
// build; `{graph_definition}` writes the working wiring graph
// unconditionally, no validation. Both variants may optionally carry
// `name`/`description` (folded in per
// agents/decisions/0012-draft-name-description-editable.md, superseding the
// removed standalone PATCH endpoint) — so a save that includes a rename
// needs to invalidate every query key that could be backing a merged
// draft+published view, same set the old PATCH mutation used to invalidate:
// this draft's own byId key (the properties panel's useGetTransformDraft),
// the bucket-3 byId key defensively, and *all*
// transformDrafts.resolve(...) keys (the left sidebar's batched resolve,
// keyed by the full sorted id list rather than this one id, so a predicate
// match is used instead of an exact key).
export function useSaveTransform(transformId: number) {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (params: SaveDraftParams) => apiSaveTransform(transformId, params),
    onSuccess: (draft) => {
      useTransformDraftStore.getState().upsertDraft(draft);
      qc.invalidateQueries({ queryKey: QUERY_KEYS.transforms.byId(transformId) });
      qc.invalidateQueries({ queryKey: QUERY_KEYS.transformDrafts.byId(transformId) });
      qc.invalidateQueries({ predicate: (q) => q.queryKey[0] === "transform-draft" && q.queryKey[1] === "resolve" });
    },
  });
}

// Standalone compile check (the code editor's "Check" button) — doesn't
// save. Replaces the removed ticket-based Compile flow; reuses the same
// synchronous compiler diagnostics Save's own rejection surfaces.
export function useValidateTransformSourceCode(transformId: number) {
  return useMutation({
    mutationFn: (source_code: string) => apiValidateTransformSourceCode(transformId, source_code),
  });
}

// Standalone composite graph validation (the composite canvas's "Validate"
// button) — validates the live in-progress canvas graph, not whatever's
// persisted. Doesn't save or persist anything; the resulting ports are
// transient/display-only.
export function useValidateCompositeGraph(transformId: number) {
  return useMutation({
    mutationFn: (graph: CompositeGraphDefinition) => apiValidateCompositeGraph(transformId, graph),
  });
}

// Bucket 2 — publish (route lives under /draft_transforms/*, despite the
// name). Bundles whatever's currently saved into the live artifact; never
// compiles. Takes the discriminated union PublishDraftParams — each call
// site already knows its own kind and passes {kind: "primitive"} or
// {kind: "composite"}.
export function usePublishTransform(transformId: number) {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (params: PublishDraftParams) => apiPublishTransform(transformId, params),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: QUERY_KEYS.transforms.all() });
      qc.invalidateQueries({ queryKey: QUERY_KEYS.transforms.byId(transformId) });
    },
  });
}

// Draft deletion — only allowed server-side for transforms that have never
// been published; the backend returns 409 otherwise (surfaced as
// mutation.error.message). See
// agents/decisions/0002-transform-draft-lifecycle-decisions.md.
export function useDeleteTransform() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (transformId: number) => apiDeleteTransform(transformId),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: QUERY_KEYS.transforms.all() });
    },
  });
}
