import { useCallback } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { apiGetTransformBinaries } from "@/Services/TransformService";
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

    for (const transformId of missingIds) {
      setStatus(transformId, "fetching");
    }

    try {
      const resolvedBinaries = await apiGetTransformBinaries(missingIds);

      for (const transformId of missingIds) {
        const binary = resolvedBinaries.get(transformId);
        if (!binary) {
          setStatus(transformId, "error");
          continue;
        }

        queryClient.setQueryData(QUERY_KEYS.transforms.wasm(transformId), binary);
        setBinary(transformId, binary);
      }
    } catch (error) {
      for (const transformId of missingIds) {
        setStatus(transformId, "error");
      }
      throw error;
    }
  }, [queryClient]);
}
