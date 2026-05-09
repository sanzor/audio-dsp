import { useCallback } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { apiGetTransformBinary } from "@/Services/TransformService";
import { useWasmBinaryStore } from "@/Stores/WasmBinaryStore";
import { QUERY_KEYS } from "@/constants/queryKeys";

export function useEnsureTransformBinaries(): (transformIds: number[]) => Promise<void> {
  const queryClient = useQueryClient();

  return useCallback(async (transformIds: number[]) => {
    const uniqueIds = Array.from(new Set(transformIds));
    const { binaries, setBinary, setStatus } = useWasmBinaryStore.getState();

    const missingIds = uniqueIds.filter((transformId) => !binaries.has(transformId));
    if (missingIds.length === 0) {
      return;
    }

    await Promise.all(
      missingIds.map(async (transformId) => {
        setStatus(transformId, "fetching");

        try {
          const binary = await queryClient.fetchQuery({
            queryKey: QUERY_KEYS.transforms.wasm(transformId),
            queryFn: () => apiGetTransformBinary(transformId),
            staleTime: Infinity,
          });
          setBinary(transformId, binary);
        } catch (error) {
          setStatus(transformId, "error");
          throw error;
        }
      })
    );
  }, [queryClient]);
}
