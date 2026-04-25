BEGIN;

-- Inject source + sink nodes into every existing graph that doesn't have them yet
UPDATE graphs
SET graph_state = jsonb_set(
    graph_state,
    '{nodes}',
    '[{"id":-1,"nodeType":"source","transformId":null,"position":{"x":0,"y":0},"params":{}},{"id":-2,"nodeType":"sink","transformId":null,"position":{"x":600,"y":0},"params":{}}]'::jsonb
    || (graph_state -> 'nodes')
)
WHERE NOT EXISTS (
    SELECT 1
    FROM jsonb_array_elements(graph_state -> 'nodes') AS node
    WHERE node ->> 'nodeType' IN ('source', 'sink')
);

-- New graphs get source + sink by default
ALTER TABLE graphs
    ALTER COLUMN graph_state SET DEFAULT
        '{"schemaVersion":1,"nodes":[{"id":-1,"nodeType":"source","transformId":null,"position":{"x":0,"y":0},"params":{}},{"id":-2,"nodeType":"sink","transformId":null,"position":{"x":600,"y":0},"params":{}}],"edges":[]}'::jsonb;

COMMIT;
