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
import { apiGetTransformWasm } from '@/Services/TransformService';
import { QUERY_KEYS } from '@/constants/queryKeys';

export function usePreloadWasmBinaries(): void {
  const transforms = useTransformStore((s) => s.transforms);
  const setBinary  = useWasmBinaryStore((s) => s.setBinary);
  const setStatus  = useWasmBinaryStore((s) => s.setStatus);

  const transformIds = [...transforms.keys()];

  useQueries({
    queries: transformIds.map((id) => ({
      queryKey: QUERY_KEYS.transforms.wasm(id),
      queryFn: async () => {
        setStatus(id, 'fetching');
        const binary = await apiGetTransformWasm(id);
        setBinary(id, binary);
        return binary;
      },
      staleTime: Infinity,
      onError: () => setStatus(id, 'error'),
    })),
  });
}
