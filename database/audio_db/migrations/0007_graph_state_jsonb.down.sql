BEGIN;

DROP FUNCTION IF EXISTS get_project_workspace(INTEGER);

CREATE TABLE IF NOT EXISTS graph_nodes (
  node_id SERIAL PRIMARY KEY,
  graph_id INTEGER NOT NULL REFERENCES graphs(graph_id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_graph_nodes_graph_id ON graph_nodes(graph_id);

CREATE TABLE IF NOT EXISTS graph_edges (
  edge_id SERIAL PRIMARY KEY,
  graph_id INTEGER NOT NULL REFERENCES graphs(graph_id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_graph_edges_graph_id ON graph_edges(graph_id);

INSERT INTO graph_nodes (node_id, graph_id, created_at)
SELECT
    (node ->> 'id')::INTEGER,
    g.graph_id,
    COALESCE(g.updated_at, g.created_at)
FROM graphs g
CROSS JOIN LATERAL jsonb_array_elements(COALESCE(g.graph_state -> 'nodes', '[]'::jsonb)) node
WHERE (node ->> 'id') ~ '^[0-9]+$'
ON CONFLICT (node_id) DO NOTHING;

INSERT INTO graph_edges (edge_id, graph_id, created_at)
SELECT
    (edge ->> 'id')::INTEGER,
    g.graph_id,
    COALESCE(g.updated_at, g.created_at)
FROM graphs g
CROSS JOIN LATERAL jsonb_array_elements(COALESCE(g.graph_state -> 'edges', '[]'::jsonb)) edge
WHERE (edge ->> 'id') ~ '^[0-9]+$'
ON CONFLICT (edge_id) DO NOTHING;

SELECT setval(
    'graph_nodes_node_id_seq',
    COALESCE((SELECT MAX(node_id) FROM graph_nodes), 1),
    (SELECT COUNT(*) > 0 FROM graph_nodes)
);

SELECT setval(
    'graph_edges_edge_id_seq',
    COALESCE((SELECT MAX(edge_id) FROM graph_edges), 1),
    (SELECT COUNT(*) > 0 FROM graph_edges)
);

ALTER TABLE graphs DROP CONSTRAINT IF EXISTS graphs_graph_state_shape_chk;

ALTER TABLE graphs
    DROP COLUMN IF EXISTS graph_state,
    DROP COLUMN IF EXISTS version,
    DROP COLUMN IF EXISTS updated_at;

CREATE OR REPLACE FUNCTION get_project_workspace(p_project_id INTEGER)
RETURNS TABLE (
    track_id        INTEGER,
    name            TEXT,
    extension       TEXT,
    length_seconds  REAL,
    region_sets     JSON
)
LANGUAGE sql
STABLE
AS $$
    SELECT
        t.track_id,
        t.name,
        t.extension,
        t.length_seconds,
        COALESCE(
            json_agg(
                jsonb_build_object(
                    'region_set_id', rs.region_set_id,
                    'track_id',      rs.track_id,
                    'track_length',  rs.track_length_seconds,
                    'name',          rs.name,
                    'regions', (
                        SELECT COALESCE(
                            json_agg(
                                jsonb_build_object(
                                    'region_id',     r.region_id,
                                    'region_set_id', r.region_set_id,
                                    'name',          r.name,
                                    'start_time',    r.start_time_seconds,
                                    'end_time',      r.end_time_seconds,
                                    'graph', (
                                        SELECT jsonb_build_object(
                                            'graph_id',  g.graph_id,
                                            'region_id', g.region_id,
                                            'name',      g.name,
                                            'nodes', COALESCE(
                                                (SELECT json_agg(jsonb_build_object('id', n.node_id, 'graph_id', n.graph_id))
                                                 FROM graph_nodes n WHERE n.graph_id = g.graph_id),
                                                '[]'::json
                                            ),
                                            'edges', COALESCE(
                                                (SELECT json_agg(jsonb_build_object('id', e.edge_id, 'graph_id', e.graph_id))
                                                 FROM graph_edges e WHERE e.graph_id = g.graph_id),
                                                '[]'::json
                                            )
                                        )
                                        FROM graphs g WHERE g.region_id = r.region_id
                                    )
                                )
                                ORDER BY r.start_time_seconds
                            ),
                            '[]'::json
                        )
                        FROM regions r WHERE r.region_set_id = rs.region_set_id
                    )
                )
            ) FILTER (WHERE rs.region_set_id IS NOT NULL),
            '[]'::json
        ) AS region_sets
    FROM tracks t
    LEFT JOIN region_sets rs ON rs.track_id = t.track_id
    WHERE t.project_id = p_project_id
    GROUP BY t.track_id, t.name, t.extension, t.length_seconds, t.created_at
    ORDER BY t.created_at ASC;
$$;

COMMIT;
