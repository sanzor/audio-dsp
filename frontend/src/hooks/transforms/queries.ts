import { useEffect } from "react";
import { useInfiniteQuery, useQuery } from "@tanstack/react-query";
import { apiGetTransforms, apiGetTransformById } from "@/Services/TransformService";
import { useTransformStore } from "@/Stores/TransformStore";
import { QUERY_KEYS } from "@/constants/queryKeys";
import type { Transform } from "@/domain/Transform/Transform";

const PAGE_SIZE = 20;

export const useListTransforms = () => {
  const query = useInfiniteQuery({
    queryKey: QUERY_KEYS.transforms.all(),
    queryFn: ({ pageParam = 0 }) => apiGetTransforms(pageParam as number, PAGE_SIZE),
    initialPageParam: 0,
    getNextPageParam: (lastPage, allPages) => {
      const loaded = allPages.flatMap((p) => p.transforms).length;
      return loaded < lastPage.total ? loaded : undefined;
    },
  });

  useEffect(() => {
    if (query.data) {
      const all = query.data.pages.flatMap((p) => p.transforms);
      useTransformStore.getState().setTransforms(all);
    }
  }, [query.data]);

  return query;
};

export const useGetTransform = (transform_id: number) => {
  const query = useQuery<Transform>({
    queryKey: QUERY_KEYS.transforms.byId(transform_id),
    queryFn: () => apiGetTransformById(transform_id),
    enabled: !!transform_id,
  });

  useEffect(() => {
    if (query.data) {
      useTransformStore.getState().setTransforms([
        ...Array.from(useTransformStore.getState().transforms.values()),
        query.data,
      ]);
    }
  }, [query.data]);

  return query;
};
