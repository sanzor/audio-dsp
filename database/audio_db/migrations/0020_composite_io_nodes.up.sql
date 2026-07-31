-- Converts the composite transform I/O model from a separate
-- `exposed_ports` list to literal Input/Output nodes participating in
-- `edges` like any other node (see composite_validator.rs and
-- frontend/src/domain/Transform/CompositeGraphDefinition.ts). This is a
-- pure JSON-shape data migration of `graph_definition` — no schema/column
-- change, `graph_definition` stays JSONB on both tables.
--
-- Narrowly targets the two known rows this shape currently exists in
-- (transform_id = 32, "Vocal Chain", in both transform_draft and
-- transform_composite) rather than a generic old->new converter for shapes
-- not otherwise observed in this environment — see the composite-io-nodes
-- feature scope notes for why a broader backfill wasn't built. The
-- `graph_definition ? 'exposed_ports'` guard additionally makes this a
-- no-op against a row that's already been converted (or created fresh
-- under the new shape), so re-running this migration is safe.

UPDATE transform_draft
SET graph_definition = '{
  "nodes": [
    {"node_kind": "leaf", "node_id": 1, "transform_id": 3},
    {"node_kind": "leaf", "node_id": 3, "transform_id": 5},
    {"node_kind": "leaf", "node_id": 4, "transform_id": 1},
    {"node_kind": "input", "node_id": 5, "name": "In"},
    {"node_kind": "output", "node_id": 6, "name": "Out"}
  ],
  "edges": [
    {"to_port": "In", "from_port": "Out", "to_node_id": 4, "from_node_id": 1},
    {"to_port": "In", "from_port": "Out", "to_node_id": 3, "from_node_id": 4},
    {"to_port": "In", "from_port": "signal", "to_node_id": 1, "from_node_id": 5},
    {"to_port": "signal", "from_port": "Out", "to_node_id": 6, "from_node_id": 3}
  ]
}'::jsonb
WHERE transform_id = 32
  AND graph_definition IS NOT NULL
  AND graph_definition ? 'exposed_ports';

UPDATE transform_composite
SET graph_definition = '{
  "nodes": [
    {"node_kind": "leaf", "node_id": 1, "transform_id": 3},
    {"node_kind": "leaf", "node_id": 2, "transform_id": 4},
    {"node_kind": "leaf", "node_id": 3, "transform_id": 5},
    {"node_kind": "input", "node_id": 4, "name": "In"},
    {"node_kind": "output", "node_id": 5, "name": "Out"}
  ],
  "edges": [
    {"to_port": "In", "from_port": "Out", "to_node_id": 2, "from_node_id": 1},
    {"to_port": "In", "from_port": "Out", "to_node_id": 3, "from_node_id": 2},
    {"to_port": "In", "from_port": "signal", "to_node_id": 1, "from_node_id": 4},
    {"to_port": "signal", "from_port": "Out", "to_node_id": 5, "from_node_id": 3}
  ]
}'::jsonb
WHERE transform_id = 32
  AND graph_definition ? 'exposed_ports';
