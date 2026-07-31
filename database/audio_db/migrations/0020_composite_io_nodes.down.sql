-- Reverts the two known transform_id = 32 rows back to the pre-Input/
-- Output-node exposed_ports shape. Semantic round-trip, not a byte-for-byte
-- one: the fresh Input/Output node_ids (5/6 in transform_draft, 4/5 in
-- transform_composite) introduced by the up migration are simply discarded
-- here rather than reconstructed, since the old shape has no node for them
-- to occupy — the remaining leaf nodes/edges and exposed_ports are restored
-- exactly to their original values either way.

UPDATE transform_draft
SET graph_definition = '{
  "edges": [
    {"to_port": "In", "from_port": "Out", "to_node_id": 4, "from_node_id": 1},
    {"to_port": "In", "from_port": "Out", "to_node_id": 3, "from_node_id": 4}
  ],
  "nodes": [
    {"node_id": 1, "transform_id": 3},
    {"node_id": 3, "transform_id": 5},
    {"node_id": 4, "transform_id": 1}
  ],
  "exposed_ports": [
    {"node_id": 1, "port_name": "In", "exposed_name": "In"},
    {"node_id": 3, "port_name": "Out", "exposed_name": "Out"}
  ]
}'::jsonb
WHERE transform_id = 32
  AND NOT (graph_definition ? 'exposed_ports');

UPDATE transform_composite
SET graph_definition = '{
  "edges": [
    {"to_port": "In", "from_port": "Out", "to_node_id": 2, "from_node_id": 1},
    {"to_port": "In", "from_port": "Out", "to_node_id": 3, "from_node_id": 2}
  ],
  "nodes": [
    {"node_id": 1, "transform_id": 3},
    {"node_id": 2, "transform_id": 4},
    {"node_id": 3, "transform_id": 5}
  ],
  "exposed_ports": [
    {"node_id": 1, "port_name": "In", "exposed_name": "In"},
    {"node_id": 3, "port_name": "Out", "exposed_name": "Out"}
  ]
}'::jsonb
WHERE transform_id = 32
  AND NOT (graph_definition ? 'exposed_ports');
