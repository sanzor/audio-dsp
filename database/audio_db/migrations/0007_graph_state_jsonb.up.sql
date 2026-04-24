BEGIN;

ALTER TABLE graphs
    ADD COLUMN graph_state JSONB,
    ADD COLUMN version INTEGER NOT NULL DEFAULT 1,
    ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT now();

UPDATE graphs g
SET
    graph_state = jsonb_build_object(
        'schemaVersion', 1,
        'nodes', COALESCE(
            (
                SELECT jsonb_agg(
                    jsonb_build_object(
                        'id', n.node_id,
                        'transformId', NULL,
                        'position', jsonb_build_object('x', 0, 'y', 0),
                        'params', jsonb_build_object()
                    )
                    ORDER BY n.created_at ASC
                )
                FROM graph_nodes n
                WHERE n.graph_id = g.graph_id
            ),
            '[]'::jsonb
        ),
        -- Existing edge rows do not carry topology data, so migrate them as empty.
        'edges', '[]'::jsonb
    ),
    updated_at = created_at
WHERE graph_state IS NULL;

ALTER TABLE graphs
    ALTER COLUMN graph_state SET NOT NULL,
    ALTER COLUMN graph_state SET DEFAULT '{"schemaVersion":1,"nodes":[],"edges":[]}'::jsonb;

ALTER TABLE graphs
    ADD CONSTRAINT graphs_graph_state_shape_chk
    CHECK (
        jsonb_typeof(graph_state) = 'object'
        AND graph_state ? 'nodes'
        AND jsonb_typeof(graph_state -> 'nodes') = 'array'
        AND graph_state ? 'edges'
        AND jsonb_typeof(graph_state -> 'edges') = 'array'
    );

DROP TABLE IF EXISTS graph_edges;
DROP TABLE IF EXISTS graph_nodes;

DROP FUNCTION IF EXISTS get_project_workspace(INTEGER);

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
                                            'graph_id',    g.graph_id,
                                            'region_id',   g.region_id,
                                            'name',        g.name,
                                            'version',     g.version,
                                            'created_at',  g.created_at,
                                            'updated_at',  g.updated_at,
                                            'graph_state', g.graph_state
                                        )
                                        FROM graphs g
                                        WHERE g.region_id = r.region_id
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
