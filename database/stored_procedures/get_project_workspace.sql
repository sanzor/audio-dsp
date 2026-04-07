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
