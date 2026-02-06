import { useEffect } from "react";
import { useAuth } from "@/Auth/UseAuth";
import type { Graph } from "@/Domain/Graph/Graph";
import type { NormalizedGraph } from "@/Domain/Graph/NormalizedGraph";
import { apiGetGraph } from "@/Services/GraphService";
import { useGraphStore } from "@/Stores/GraphStore";
import { useQuery } from "@tanstack/react-query";
import { normalizeGraph } from "./utils";


export const useGetGraph = (graphId: string) => {
    // 1. Get necessary state and mutator from the Zustand store
    const cachedGraph = useGraphStore(state => state.getGraph(graphId));
    const { user } = useAuth();
    
    // We rely on React Query's built-in caching and refetch logic.
    // If 'cachedGraph' exists, the query will only run in the background if the cache time is expired.
    // However, if we rely ONLY on React Query's cache, we simplify the logic here:

    const query = useQuery<Graph, Error, NormalizedGraph | undefined>({
        queryKey: ['graph', graphId],
        queryFn: () => apiGetGraph(graphId),
        enabled: !!graphId && !!user,
        select: (data) => (data ? normalizeGraph(data) : cachedGraph),
    });

    useEffect(() => {
        if (query.data) useGraphStore.getState().addGraph(query.data);
    }, [query.data]);

    return query;
};
