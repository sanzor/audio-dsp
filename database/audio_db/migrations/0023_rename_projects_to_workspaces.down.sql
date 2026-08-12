BEGIN;

DROP FUNCTION get_workspace_tracks(INTEGER);

ALTER INDEX idx_sources_workspace_id RENAME TO idx_sources_project_id;
ALTER TABLE sources RENAME CONSTRAINT sources_workspace_id_fkey TO sources_project_id_fkey;
ALTER TABLE sources RENAME COLUMN workspace_id TO project_id;

ALTER TABLE tracks RENAME CONSTRAINT tracks_workspace_id_fkey TO tracks_project_id_fkey;
ALTER TABLE tracks RENAME COLUMN workspace_id TO project_id;

ALTER TABLE workspace_members RENAME CONSTRAINT workspace_members_role_check TO project_members_role_check;
ALTER TABLE workspace_members RENAME CONSTRAINT workspace_members_user_id_fkey TO project_members_user_id_fkey;
ALTER TABLE workspace_members RENAME CONSTRAINT workspace_members_workspace_id_fkey TO project_members_project_id_fkey;
ALTER TABLE workspace_members RENAME CONSTRAINT workspace_members_pkey TO project_members_pkey;
ALTER TABLE workspace_members RENAME COLUMN workspace_id TO project_id;
ALTER TABLE workspace_members RENAME TO project_members;

ALTER TABLE workspaces RENAME CONSTRAINT workspaces_created_by_fkey TO projects_created_by_fkey;
ALTER INDEX workspaces_pkey RENAME TO projects_pkey;
ALTER SEQUENCE workspaces_workspace_id_seq RENAME TO projects_project_id_seq;
ALTER TABLE workspaces RENAME COLUMN workspace_id TO project_id;
ALTER TABLE workspaces RENAME TO projects;

-- Function recreated last, after project_id columns exist again on every
-- table it references.
CREATE FUNCTION get_project_workspace(p_project_id INTEGER)
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
