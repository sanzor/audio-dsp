import { useMutation, useQueryClient } from "@tanstack/react-query";
import { apiCopyGraph, apiCreateGraph, apiRemoveGraph, apiUpdateGraph } from "@/Services/GraphService";
import { useGraphStore } from "@/Stores/GraphStore";
import { normalizeGraph } from "./utils";
import type { CopyGraphParams } from "@/Dtos/Graphs/CopyGraphParams";
import type { CopyGraphResult } from "@/Dtos/Graphs/CopyGraphResult";
import { useRegionStore } from "@/Stores/RegionStore";
import type { CreateGraphParams } from "@/Dtos/Graphs/CreateGraphParams";
import type { CreateGraphResult } from "@/Dtos/Graphs/CreateGraphResult";
import type { RemoveGraphParams } from "@/Dtos/Graphs/RemoveGraphParams";
import type { NormalizedGraph } from "@/Domain/Graph/NormalizedGraph";
import type { EditGraphParams } from "@/Dtos/Graphs/EditGraphParams";
import type { EditGraphResult } from "@/Dtos/Graphs/EditGraphResult";

export const useCreateGraph = () => {
  const addGraph = useGraphStore.getState().addGraph;
  const setGraph = useRegionStore.getState().setGraph;
  return useMutation<CreateGraphResult, Error, CreateGraphParams>({
    mutationFn: (params) => apiCreateGraph(params),
    onSuccess: (data) => {
      //update region set , attach region
      const graph=data.graph;
      const normalizedGraph=normalizeGraph(graph);
      addGraph(normalizedGraph);
      setGraph(normalizedGraph.regionId,normalizedGraph.id);
    },
    onError: (error) => {
      console.error('Failed to create region', error);
    },
  });
};


export const useCopyGraph = () => {
  const queryClient = useQueryClient();
  const addGraph = useGraphStore.getState().addGraph;
  const setGraph = useRegionStore.getState().setGraph;

  return useMutation<CopyGraphResult, Error, CopyGraphParams>({
    mutationFn: params => apiCopyGraph(params),
    onSuccess: (data) => {
      const normalized = normalizeGraph(data.graph);
      addGraph(normalized);
      setGraph(normalized.regionId, normalized.id);
      queryClient.invalidateQueries({ queryKey: ['graph', normalized.id] });
    },
    onError: (error) => {
      console.error("Failed to copy graph", error);
    },
  });
};

export const useDeleteGraph = () => {
  const getGraph = useGraphStore.getState().getGraph;
  const removeGraph = useGraphStore.getState().removeGraph;
  const updateRegion = useRegionStore.getState().updateRegion;

  return useMutation<
    void,
    Error,
    RemoveGraphParams,
    { previousGraph?: NormalizedGraph }
  >({
    mutationFn: params => apiRemoveGraph(params),
    onMutate: params => {
      const previousGraph = getGraph(params.graph_id);

      if (previousGraph) {
        removeGraph(previousGraph.id);
        updateRegion(previousGraph.regionId, { graphId: null });
      }

      return { previousGraph };
    },
    onError: (
      _error: Error,
      _params: RemoveGraphParams,
      ctx: { previousGraph?: NormalizedGraph } | undefined
    ) => {
      if (ctx?.previousGraph) {
        useGraphStore.getState().addGraph(ctx.previousGraph);
        useRegionStore
          .getState()
          .updateRegion(ctx.previousGraph.regionId, {
            graphId: ctx.previousGraph.id,
          });
      }
    },
  });
};

export const useEditGraph = () => {
  const editGraph = useGraphStore.getState().updateGraph;
  return useMutation<EditGraphResult, Error, EditGraphParams>({
    mutationFn: (params) => apiUpdateGraph(params),
    onSuccess: (data) => {
       const graph=data.graph;
       const normalizedGraph=normalizeGraph(graph);
       editGraph(graph.id,normalizedGraph);
    },
    onError: (error) => {
      console.error('Failed to edit region', error);
    },
  });
};

/**
 * Facade hook that bundles all graph mutations
 */
export const useGraphMutations = () => ({
  create: useCreateGraph(),
  copy: useCopyGraph(),
  edit: useEditGraph(),
  remove: useDeleteGraph(),
});
