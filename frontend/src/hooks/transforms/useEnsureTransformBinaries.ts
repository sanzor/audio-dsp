import { useCallback } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { apiGetTransformBinaries } from "@/Services/TransformService";
import { useWasmBinaryStore } from "@/Stores/WasmBinaryStore";
import { QUERY_KEYS } from "@/constants/queryKeys";

export function useEnsureTransformBinaries(): (transformIds: number[]) => Promise<Map<number, Uint8Array>> {
  const queryClient = useQueryClient();

  return useCallback(async (transformIds: number[]) => {
    const uniqueIds = Array.from(new Set(transformIds));
    const { binaries, setBinary, setStatus } = useWasmBinaryStore.getState();
    const resolved = new Map<number, Uint8Array>();

    const missingIds: number[] = [];
    for (const transformId of uniqueIds) {
      const existingBinary = binaries.get(transformId);
      if (existingBinary) {
        resolved.set(transformId, existingBinary);
        continue;
      }

      const cachedBinary = queryClient.getQueryData<Uint8Array>(QUERY_KEYS.transforms.wasm(transformId));
      if (cachedBinary) {
        setBinary(transformId, cachedBinary);
        resolved.set(transformId, cachedBinary);
        continue;
      }

      missingIds.push(transformId);
    }

    if (missingIds.length === 0) {
      return resolved;
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
        resolved.set(transformId, binary);
      }
    } catch (error) {
      for (const transformId of missingIds) {
        setStatus(transformId, "error");
      }
      throw error;
    }

    return resolved;
  }, [queryClient]);
}
