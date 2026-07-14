import { useEffect, useMemo } from "react";
import { useInfiniteQuery, useQuery, useQueryClient } from "@tanstack/react-query";
import {
  apiGetCompileTicketStatus,
  apiGetTransformBinaries,
  apiGetTransformDefinition,
  apiGetTransformSummaries,
} from "@/Services/TransformService";
import { useTransformStore } from "@/Stores/TransformStore";
import { useWasmBinaryStore } from "@/Stores/WasmBinaryStore";
import { useAuthStore } from "@/Stores/authStore";
import { useProjectStore } from "@/Stores/projectStore";
import { QUERY_KEYS } from "@/constants/queryKeys";
import type { Node as RFNode } from "reactflow";
import type { TransformDefinition } from "@/domain/Transform/Transform";

const PAGE_SIZE = 20;

function getTransformBinaries(transformIds: number[]): Map<number, Uint8Array> {
  const binaries = useWasmBinaryStore((s) => s.binaries)

  const missingIds = useMemo(
    () => transformIds.filter((id) => !binaries.has(id)),
    [transformIds, binaries],
  )

  const query = useQuery({
    queryKey: QUERY_KEYS.transforms.wasmBatch(missingIds),
    queryFn: () => apiGetTransformBinaries(missingIds),
    enabled: missingIds.length > 0,
    staleTime: Infinity,
  })

  useEffect(() => {
    if (!query.data) return
    for (const [id, binary] of query.data.entries()) {
      useWasmBinaryStore.getState().setBinary(id, binary)
    }
  }, [query.data])

  return binaries
}

export function useGraphBinaries(nodes: RFNode[]): Map<number, Uint8Array> {
  const transformIds = useMemo(
    () => [...new Set(nodes.flatMap((n) => {
      const id = n.data.transformId as number | null | undefined
      return id != null ? [id] : []
    }))],
    [nodes],
  )
  return getTransformBinaries(transformIds)
}

export function usePreloadBinaries(transformIds: number[]): void {
  getTransformBinaries(transformIds)
}

export const useListTransforms = () => {
  const user = useAuthStore((state) => state.user);
  const activeProjectId = useProjectStore((state) => state.activeProject?.project_id);
  const query = useInfiniteQuery({
    queryKey: [...QUERY_KEYS.transforms.all(), activeProjectId ?? 0],
    queryFn: ({ pageParam = 0 }) => apiGetTransformSummaries(pageParam as number, PAGE_SIZE),
    enabled: Boolean(user && activeProjectId),
    initialPageParam: 0,
    getNextPageParam: (lastPage, allPages) => {
      const loaded = allPages.flatMap((p) => p.transforms).length;
      return loaded < lastPage.total ? loaded : undefined;
    },
  });

  useEffect(() => {
    if (query.data) {
      const all = query.data.pages.flatMap((p) => p.transforms);
      useTransformStore.getState().setSummaries(all);
    }
  }, [query.data]);

  return query;
};

export const useGetTransformDefinition = (
  transform_id: number | null | undefined,
  enabled = true
) => {
  const user = useAuthStore((state) => state.user);
  const activeProjectId = useProjectStore((state) => state.activeProject?.project_id);
  const resolvedTransformId = transform_id ?? -1;
  const query = useQuery<TransformDefinition>({
    queryKey: [...QUERY_KEYS.transforms.byId(resolvedTransformId), activeProjectId ?? 0],
    queryFn: () => apiGetTransformDefinition(resolvedTransformId),
    enabled: transform_id != null && enabled && Boolean(user && activeProjectId),
  });

  useEffect(() => {
    if (query.data) {
      useTransformStore.getState().upsertDefinition(query.data);
    }
  }, [query.data]);

  return query;
};

// Polls a compile ticket until it reaches a terminal state (successful/failed).
// On success, invalidates the transform's cached binary and definition so the
// newly compiled artifact gets refetched.
export function useCompileTicketStatus(ticketId: number | null, transformId: number | null) {
  const qc = useQueryClient();
  const query = useQuery({
    queryKey: QUERY_KEYS.transforms.ticket(ticketId ?? -1),
    queryFn: () => apiGetCompileTicketStatus(ticketId!),
    enabled: ticketId != null,
    refetchInterval: (q) => (q.state.data?.status.state === "processing" ? 1500 : false),
  });

  useEffect(() => {
    if (query.data?.status.state === "successful" && transformId != null) {
      qc.invalidateQueries({ queryKey: ["transform", "wasm"] });
      qc.invalidateQueries({ queryKey: QUERY_KEYS.transforms.byId(transformId) });
    }
  }, [query.data?.status.state, transformId, qc]);

  return query;
}
