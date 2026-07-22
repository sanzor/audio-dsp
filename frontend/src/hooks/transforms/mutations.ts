import { useMutation, useQueryClient } from "@tanstack/react-query";
import {
  apiCreateTransform,
  apiSaveTransform,
  apiPublishTransform,
  apiDeleteTransform,
  type CreateTransformParams,
  type SaveTransformParams,
} from "@/Services/TransformService";
import { QUERY_KEYS } from "@/constants/queryKeys";

export function useCreateTransform() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (params: CreateTransformParams) => apiCreateTransform(params),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: QUERY_KEYS.transforms.all() });
    },
  });
}

// Bucket 2 — save. Independent of compiling — safe to call whether or not a
// compile ticket is in flight for this transform.
export function useSaveTransform(transformId: number) {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (params: SaveTransformParams) => apiSaveTransform(transformId, params),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: QUERY_KEYS.transforms.byId(transformId) });
    },
  });
}

// Bucket 3 — publish. Bundles whatever's currently saved into the live
// artifact; does not compile.
export function usePublishTransform(transformId: number) {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: () => apiPublishTransform(transformId),
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
