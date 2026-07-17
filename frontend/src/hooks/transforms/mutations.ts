import { useMutation, useQueryClient } from "@tanstack/react-query";
import {
  apiCreateTransform,
  apiUpdateTransform,
  type CreateTransformParams,
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

export interface SaveTransformParams {
  name: string;
  description?: string;
}

// Ports/params are read-only here — they're derived from source on compile
// (see TransformsDataProvider::publish_compiled_transform), not editable metadata.
export function useSaveTransform(transformId: number) {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ name, description }: SaveTransformParams) =>
      apiUpdateTransform(transformId, { name, description }),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: QUERY_KEYS.transforms.all() });
      qc.invalidateQueries({ queryKey: QUERY_KEYS.transforms.byId(transformId) });
    },
  });
}
