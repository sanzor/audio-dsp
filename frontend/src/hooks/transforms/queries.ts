import { useEffect } from "react";
import { useInfiniteQuery, useQuery } from "@tanstack/react-query";
import { apiGetTransformDefinition, apiGetTransformSummaries } from "@/Services/TransformService";
import { useTransformStore } from "@/Stores/TransformStore";
import { useAuthStore } from "@/Stores/authStore";
import { useProjectStore } from "@/Stores/projectStore";
import { QUERY_KEYS } from "@/constants/queryKeys";
import type { TransformDefinition } from "@/domain/Transform/Transform";

const PAGE_SIZE = 20;

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
