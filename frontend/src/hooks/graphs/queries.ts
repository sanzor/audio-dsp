import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { useAuth } from "@/Auth/UseAuth";
import { useGraphStore } from "@/Stores/GraphStore";
import { apiGetGraph } from "@/Services/GraphService";
import { QUERY_KEYS } from "@/constants/queryKeys";
import type { Graph } from "@/domain/Graph/Graph";

export const useGetGraph = (graphId: string) => {
  const cachedGraph = useGraphStore((state) => state.getGraph(graphId));
  const { user } = useAuth();

  const query = useQuery<Graph, Error, Graph | undefined>({
    queryKey: QUERY_KEYS.graphs.byId(graphId),
    queryFn: () => apiGetGraph(graphId),
    enabled: !!graphId && !!user,
    select: (data) => data ?? cachedGraph,
  });

  useEffect(() => {
    if (query.data) useGraphStore.getState().addGraph(query.data);
  }, [query.data]);

  return query;
};
