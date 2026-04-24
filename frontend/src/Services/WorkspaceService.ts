import type { Edge } from "@/domain/Graph/Edge";
import type { Graph } from "@/domain/Graph/Graph";
import type { Node } from "@/domain/Graph/Node";
import type { TrackRegion } from "@/domain/Region/TrackRegion";
import type { TrackRegionSet } from "@/domain/RegionSet/TrackRegionSet";
import type { TrackMeta } from "@/domain/Track/TrackMeta";
import { http } from "@/Services/http";

interface ApiWorkspaceGraphNode {
  id: number;
  graph_id?: number;
  transformId?: number | null;
  position?: { x?: number; y?: number };
  params?: Record<string, number>;
}

interface ApiWorkspaceGraphEdge {
  id: number;
  graph_id?: number;
  fromNodeId?: number;
  toNodeId?: number;
  fromPortId?: number | null;
  toPortId?: number | null;
  to?: string;
}

interface ApiWorkspaceGraphState {
  schemaVersion?: number;
  nodes?: ApiWorkspaceGraphNode[];
  edges?: ApiWorkspaceGraphEdge[];
}

interface ApiWorkspaceGraph {
  graph_id: number;
  region_id?: number | null;
  name: string;
  repr?: ApiWorkspaceGraphState;
  graph_state?: ApiWorkspaceGraphState;
  nodes?: ApiWorkspaceGraphNode[];
  edges?: ApiWorkspaceGraphEdge[];
  created_at?: string;
  updated_at?: string;
}

interface ApiWorkspaceRegion {
  region_id: number;
  region_set_id: number;
  name: string;
  start_time: number;
  end_time: number;
  graph?: ApiWorkspaceGraph | null;
}

interface ApiWorkspaceRegionSet {
  track_id: number;
  track_length: number;
  region_set_id: number;
  name: string;
  regions: ApiWorkspaceRegion[];
}

interface ApiWorkspaceTrack {
  track_id: number;
  name: string;
  extension: string;
  length_seconds: number;
  region_sets: ApiWorkspaceRegionSet[];
}

const mapWorkspaceNode = (node: ApiWorkspaceGraphNode, graphId: number): Node => ({
  id: node.id,
  graphId: node.graph_id ?? graphId,
  transformId: node.transformId ?? null,
  position: { x: node.position?.x ?? 0, y: node.position?.y ?? 0 },
  params: node.params ?? {},
  ports: [],
  createdAt: new Date(0),
});

const mapWorkspaceEdge = (edge: ApiWorkspaceGraphEdge, graphId: number): Edge => ({
  id: edge.id,
  graphId: edge.graph_id ?? graphId,
  fromNodeId: edge.fromNodeId ?? 0,
  toNodeId: edge.toNodeId ?? 0,
  fromPortId: edge.fromPortId ?? null,
  toPortId: edge.toPortId ?? null,
  to: edge.to ?? "",
});

const mapWorkspaceGraph = (graph: ApiWorkspaceGraph, regionId: number): Graph => {
  const repr = graph.repr ?? graph.graph_state;
  const nodes = repr?.nodes ?? graph.nodes ?? [];
  const edges = repr?.edges ?? graph.edges ?? [];

  return {
    id: graph.graph_id,
    regionId: graph.region_id ?? regionId,
    name: graph.name,
    createdAt: graph.created_at ? new Date(graph.created_at) : new Date(0),
    updatedAt: graph.updated_at ? new Date(graph.updated_at) : new Date(0),
    nodes: nodes.map((node) => mapWorkspaceNode(node, graph.graph_id)),
    edges: edges.map((edge) => mapWorkspaceEdge(edge, graph.graph_id)),
  };
};

const mapWorkspaceRegion = (region: ApiWorkspaceRegion): TrackRegion => ({
  regionId: region.region_id,
  regionSetId: region.region_set_id,
  name: region.name,
  start: region.start_time,
  end: region.end_time,
  graph: region.graph ? mapWorkspaceGraph(region.graph, region.region_id) : undefined,
});

const mapWorkspaceRegionSet = (regionSet: ApiWorkspaceRegionSet): TrackRegionSet => ({
  id: regionSet.region_set_id,
  trackId: regionSet.track_id,
  name: regionSet.name,
  regions: regionSet.regions.map(mapWorkspaceRegion),
});

const mapWorkspaceTrack = (track: ApiWorkspaceTrack): TrackMeta => ({
  trackId: track.track_id,
  trackInfo: {
    name: track.name,
    extension: track.extension,
  },
  regionSets: track.region_sets.map(mapWorkspaceRegionSet),
});

export async function apiGetWorkspace(projectId: number): Promise<TrackMeta[]> {
  const response = await http.get<ApiWorkspaceTrack[]>(`/v1/projects/${projectId}/workspace`);
  return response.map(mapWorkspaceTrack);
}
