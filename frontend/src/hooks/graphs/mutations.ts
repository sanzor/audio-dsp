import { useMutation, useQueryClient } from "@tanstack/react-query";
import {
  apiCopyGraph,
  apiCreateGraph,
  apiRemoveGraph,
  apiSaveGraphState,
  apiUpdateGraph,
  type CopyGraphParams,
  type CopyGraphResult,
  type CreateGraphParams,
  type CreateGraphResult,
  type EditGraphParams,
  type EditGraphResult,
  type RemoveGraphParams,
  type SaveGraphStateParams,
  type SaveGraphStateResult,
} from "@/Services/GraphService";
import { useGraphStore } from "@/Stores/GraphStore";
import { useRegionStore } from "@/Stores/RegionStore";
import { QUERY_KEYS } from "@/constants/queryKeys";
import type { Graph } from "@/domain/Graph/Graph";

// ─── Cascade helpers ─────────────────────────────────────────────────────────

export const cascadeDeleteGraph = (graphId: number): void => {
  useGraphStore.getState().removeGraph(graphId);
};

// ─── Mutations ───────────────────────────────────────────────────────────────

export const useCreateGraph = () => {
  const addGraph = useGraphStore.getState().addGraph;
  const setGraph = useRegionStore.getState().setGraph;

  return useMutation<CreateGraphResult, Error, CreateGraphParams>({
    mutationFn: (params) => apiCreateGraph(params),
    onSuccess: (data) => {
      addGraph(data.graph);
      setGraph(data.graph.regionId, data.graph.id);
    },
    onError: (error) => {
      console.error('Failed to create graph:', error);
    },
  });
};

export const useCopyGraph = () => {
  const queryClient = useQueryClient();
  const addGraph = useGraphStore.getState().addGraph;
  const setGraph = useRegionStore.getState().setGraph;

  return useMutation<CopyGraphResult, Error, CopyGraphParams>({
    mutationFn: (params) => apiCopyGraph(params),
    onSuccess: (data) => {
      addGraph(data.graph);
      setGraph(data.graph.regionId, data.graph.id);
      queryClient.invalidateQueries({ queryKey: QUERY_KEYS.graphs.byId(data.graph.id) });
    },
    onError: (error) => {
      console.error('Failed to copy graph:', error);
    },
  });
};

export const useDeleteGraph = () => {
  const getGraph = useGraphStore.getState().getGraph;
  const removeGraph = useGraphStore.getState().removeGraph;
  const updateRegion = useRegionStore.getState().updateRegion;

  return useMutation<void, Error, RemoveGraphParams, { previousGraph?: Graph }>({
    mutationFn: (params) => apiRemoveGraph(params),
    onMutate: (params) => {
      const previousGraph = getGraph(params.graph_id);
      if (previousGraph) {
        removeGraph(previousGraph.id);
        updateRegion(previousGraph.regionId, { graphId: null });
      }
      return { previousGraph };
    },
    onError: (_error, _params, ctx) => {
      if (ctx?.previousGraph) {
        useGraphStore.getState().addGraph(ctx.previousGraph);
        useRegionStore.getState().updateRegion(ctx.previousGraph.regionId, { graphId: ctx.previousGraph.id });
      }
    },
  });
};

export const useEditGraph = () => {
  const editGraph = useGraphStore.getState().updateGraph;

  return useMutation<EditGraphResult, Error, EditGraphParams>({
    mutationFn: (params) => apiUpdateGraph(params),
    onSuccess: (data) => {
      editGraph(data.graph.id, data.graph);
    },
    onError: (error) => {
      console.error('Failed to edit graph:', error);
    },
  });
};

export const useSaveGraphState = () => {
  const queryClient = useQueryClient();
  const updateGraph = useGraphStore.getState().updateGraph;

  return useMutation<SaveGraphStateResult, Error, SaveGraphStateParams>({
    mutationFn: (params) => apiSaveGraphState(params),
    onSuccess: (data) => {
      updateGraph(data.graph.id, data.graph);
      queryClient.invalidateQueries({ queryKey: QUERY_KEYS.graphs.byId(data.graph.id) });
    },
  });
};

export const useGraphMutations = () => ({
  create: useCreateGraph(),
  copy: useCopyGraph(),
  edit: useEditGraph(),
  remove: useDeleteGraph(),
});
