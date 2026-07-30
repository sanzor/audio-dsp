// A composite transform's wiring — the client-side mirror of the backend's
// CompositeGraphSnapshot/CompositeGraphDefinitionDto. Field names are
// snake_case to match the wire format directly, same convention as
// TransformPort/TransformParam in this domain folder (no camelCase mapping
// layer, unlike the Editor's Graph/Node/Edge types). References other
// transforms by transform_id and their ports by name (never port_id, which
// is reassigned on every republish). v1 has no per-node param overrides and
// no exposed params; only ports are exposed outward.

export interface CompositeNode {
  // Canvas-local instance id — distinct from transform_id since one leaf
  // transform can be placed as multiple instances in the same composite.
  node_id: number;
  transform_id: number;
}

export interface CompositeEdge {
  from_node_id: number;
  from_port: string;
  to_node_id: number;
  to_port: string;
}

export interface CompositeExposedPort {
  node_id: number;
  port_name: string;
  exposed_name: string;
}

export interface CompositeGraphDefinition {
  nodes: CompositeNode[];
  edges: CompositeEdge[];
  exposed_ports: CompositeExposedPort[];
}
