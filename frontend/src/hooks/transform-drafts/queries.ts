import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { apiGetTransformDraft, apiResolveTransformDrafts } from "@/Services/transform-drafts/TransformDraftsService";
import { useTransformDraftStore } from "@/Stores/transform-drafts/TransformDraftStore";
import { useAuthStore } from "@/Stores/authStore";
import { useProjectStore } from "@/Stores/projectStore";
import { QUERY_KEYS } from "@/constants/queryKeys";
import type { TransformDraftDefinition } from "@/domain/transform-drafts/TransformDraftDefinition";

// Bucket 2 — a draft's currently-persisted name/description/source/metadata,
// as opposed to hooks/transforms/queries.ts's useGetTransformDefinition
// (bucket 3, only ever reflects what was last published). Mirrors that
// hook's auth/project-gated shape. Used by the Creator's properties panel
// to source the editable name/description fields; ports/params stay
// sourced from bucket-3 (unchanged, still compile-derived/read-only).
export const useGetTransformDraft = (transformId: number | null) => {
  const user = useAuthStore((state) => state.user);
  const activeProjectId = useProjectStore((state) => state.activeProject?.project_id);
  const resolvedId = transformId ?? -1;
  return useQuery<TransformDraftDefinition>({
    queryKey: [...QUERY_KEYS.transformDrafts.byId(resolvedId), activeProjectId ?? 0],
    queryFn: () => apiGetTransformDraft(resolvedId),
    enabled: transformId != null && Boolean(user && activeProjectId),
  });
};

// Batched draft fetch for the left transforms-sidebar — resolves every
// currently-listed bucket-3 transform's draft in one call, so the sidebar
// can merge in each row's possibly-Save-edited name/description without
// depending on any backend sync of `transform` itself. Mirrors
// hooks/transforms/queries.ts's useResolveTransformDefinitions
// (bucket 3) shape/pattern. `get_transform_drafts` fails the whole batch on
// the first access-denied id (see
// agents/decisions/0012-draft-name-description-editable.md's addendum) —
// callers must fall back to bucket-3 display on error, not blank the list.
export const useResolveTransformDrafts = (transformIds: number[]) => {
  const user = useAuthStore((state) => state.user);
  const activeProjectId = useProjectStore((state) => state.activeProject?.project_id);
  const sortedIds = [...new Set(transformIds)].sort((a, b) => a - b);
  const query = useQuery<TransformDraftDefinition[]>({
    queryKey: [...QUERY_KEYS.transformDrafts.resolve(sortedIds), activeProjectId ?? 0],
    queryFn: () => apiResolveTransformDrafts(sortedIds),
    enabled: sortedIds.length > 0 && Boolean(user && activeProjectId),
  });

  useEffect(() => {
    if (query.data) {
      for (const draft of query.data) {
        useTransformDraftStore.getState().upsertDraft(draft);
      }
    }
  }, [query.data]);

  return query;
};
