BEGIN;

-- Strip source + sink nodes from all graphs
UPDATE graphs
SET graph_state = jsonb_set(
    graph_state,
    '{nodes}',
    COALESCE(
        (
            SELECT jsonb_agg(node)
            FROM jsonb_array_elements(graph_state -> 'nodes') AS node
            WHERE node ->> 'nodeType' NOT IN ('source', 'sink')
        ),
        '[]'::jsonb
    )
);

-- Restore the old default (empty nodes/edges)
ALTER TABLE graphs
    ALTER COLUMN graph_state SET DEFAULT
        '{"schemaVersion":1,"nodes":[],"edges":[]}'::jsonb;

COMMIT;
