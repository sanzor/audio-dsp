// A composite transform's wiring — the client-side mirror of the backend's
// CompositeGraphSnapshot/CompositeGraphDefinitionDto. Field names are
// snake_case to match the wire format directly, same convention as
// TransformPort/TransformParam in this domain folder (no camelCase mapping
// layer, unlike the Editor's Graph/Node/Edge types). Leaf nodes reference
// other transforms by transform_id and their ports by name (never port_id,
// which is reassigned on every republish). v1 has no per-node param
// overrides. Every node also carries its canvas `position`, persisted
// server-side (CompositeNodePositionDto) — old rows saved before this field
// existed deserialize it as `{x:0, y:0}` via the backend's `serde(default)`.

// A composite's own externally-visible ports are not a separate list —
// they're derived from the graph's Input/Output nodes below: an Input
// node's one implicit output handle wired into a leaf's real input port, or
// a leaf's real output port wired into an Output node's one implicit input
// handle. This replaced an earlier model where exposing a port meant
// marking a leaf's own (name, exposed_name) pair in a separate
// CompositeExposedPort[] list with no participation in `edges` at all.

// Mirrors backend/api/src/transform_drafts/graph_validator/node.rs's `Node`
// exactly (same `node_kind` tag field, same tag values, same per-variant
// field names, including `position`) — save/validate-graph/publish
// deserialize this directly from what this client sends, with no
// translation layer, so the two must stay in lockstep. Tag values are
// "primitive" | "composite" | "input" | "output" — a leaf's tag must match
// the *referenced transform's own* kind (validated server-side; a
// primitive leaf tagged "composite" or vice versa fails validation), not a
// generic "leaf" tag like the pre-reshape backend used. Named `node_kind`
// (not `kind`) to avoid colliding with the unrelated "primitive" |
// "composite" `kind` field that already means something else at the
// transform-definition level elsewhere in this domain folder (see
// TransformDefinition.ts) — that collision in naming is coincidental with
// this reshape now also making the *values* line up 1:1 with that same
// field for a leaf node specifically.
export type CompositeNode = CompositeLeafNode | CompositeIoNode;

export interface CompositeLeafNode {
  // Canvas-local instance id — distinct from transform_id since one leaf
  // transform can be placed as multiple instances in the same composite.
  node_id: number;
  // Must equal the referenced transform's own kind — see this file's note
  // above. Stores/CompositeCanvasStore.ts's toGraphDefinition() derives
  // this from useTransformStore at serialize time; the canvas's own
  // internal CanvasLeafNode representation stays a single "leaf" tag
  // regardless (Editor/Creator-style internal-model-vs-wire-format split,
  // not a backend concept).
  node_kind: "primitive" | "composite";
  transform_id: number;
  // Canvas position, persisted server-side. Old saved composites predating
  // this field deserialize it as `{x:0, y:0}` (backend `serde(default)`).
  position: { x: number; y: number };
}

export interface CompositeIoNode {
  node_id: number;
  node_kind: "input" | "output";
  // The externally-visible port name this node produces/consumes once
  // wired to a leaf — replaces the old CompositeExposedPort.exposed_name.
  // Must be non-empty and unique across all Input/Output nodes in the
  // graph (enforced server-side by composite_validator.rs).
  name: string;
  // Canvas position, persisted server-side. Old saved composites predating
  // this field deserialize it as `{x:0, y:0}` (backend `serde(default)`).
  position: { x: number; y: number };
}

export interface CompositeEdge {
  from_node_id: number;
  from_port: string;
  to_node_id: number;
  to_port: string;
}

// Fixed pseudo-port name used on the single implicit handle every
// Input/Output node exposes (an Input node's one output handle, an Output
// node's one input handle) — the CompositeEdge.from_port/.to_port literal
// whenever that side of the edge is an Input/Output node. Must match
// backend composite_validator.rs's IO_PORT_NAME constant exactly — both
// sides hand-authored, not shared code across the Rust/TS boundary.
export const IO_NODE_PORT_NAME = "signal";

export interface CompositeGraphDefinition {
  nodes: CompositeNode[];
  edges: CompositeEdge[];
}
