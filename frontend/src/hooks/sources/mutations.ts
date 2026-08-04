import { useMutation, useQueryClient } from "@tanstack/react-query";
import {
  apiAddSourceMulti,
  apiRenameSource,
  apiDeleteSource,
  type AddSourceMultiParams,
  type RenameSourceParams,
} from "@/Services/SourceService";
import { QUERY_KEYS } from "@/constants/queryKeys";

export function useUploadSource() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (params: AddSourceMultiParams) => apiAddSourceMulti(params),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: QUERY_KEYS.sources.all() });
    },
  });
}

export function useRenameSource() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (params: RenameSourceParams) => apiRenameSource(params),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: QUERY_KEYS.sources.all() });
    },
  });
}

export function useDeleteSource() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (sourceId: number) => apiDeleteSource(sourceId),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: QUERY_KEYS.sources.all() });
    },
  });
}
