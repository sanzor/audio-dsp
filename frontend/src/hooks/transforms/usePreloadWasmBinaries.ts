/**
 * usePreloadWasmBinaries
 *
 * Fetches the WASM binary for every transform in the TransformStore and
 * caches it in WasmBinaryStore.  Call this once after the transform list
 * has been loaded (e.g. in the dashboard layout after login).
 *
 * Uses React Query so concurrent fetches are deduplicated and results are
 * cached for the session (staleTime: Infinity — binaries don't change at runtime).
 */

import { useQueries } from '@tanstack/react-query';
import { useTransformStore } from '@/Stores/TransformStore';
import { useWasmBinaryStore } from '@/Stores/WasmBinaryStore';
import { apiGetTransformBinary } from '@/Services/TransformService';
import { QUERY_KEYS } from '@/constants/queryKeys';

export function usePreloadWasmBinaries(): void {
  const summaries = useTransformStore((s) => s.summaries);
  const setBinary  = useWasmBinaryStore((s) => s.setBinary);
  const setStatus  = useWasmBinaryStore((s) => s.setStatus);

  const transformIds = [...summaries.keys()];

  useQueries({
    queries: transformIds.map((id) => ({
      queryKey: QUERY_KEYS.transforms.wasm(id),
      queryFn: async () => {
        setStatus(id, 'fetching');
        const binary = await apiGetTransformBinary(id);
        setBinary(id, binary);
        return binary;
      },
      staleTime: Infinity,
      onError: () => setStatus(id, 'error'),
    })),
  });
}
