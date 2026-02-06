import type { Graph } from "@/Domain/Graph/Graph";
import type { NormalizedGraph } from "@/Domain/Graph/NormalizedGraph";
import { useGraphStore } from "@/Stores/GraphStore";



export const normalizeGraph = (graphApi: Graph): NormalizedGraph => {
  // Graph is the aggregate root - nodes and edges stay as a unit
  return {
    ...graphApi,
    nodes: graphApi.nodes ?? [],
    edges: graphApi.edges ?? [],
  };
};

export const cascadeDeleteGraph = (graphId: string): void => {
  const removeGraph = useGraphStore.getState().removeGraph;
  removeGraph(graphId);
};
