-- Rename the "project" concept to "workspace" across the schema. Done as a
-- single migration (not split) because get_project_workspace() is a plain-SQL
-- function resolved against column names at call time -- splitting the table
-- rename from the function fix would leave a broken intermediate state.
BEGIN;

ALTER TABLE projects RENAME TO workspaces;
ALTER TABLE workspaces RENAME COLUMN project_id TO workspace_id;
ALTER SEQUENCE projects_project_id_seq RENAME TO workspaces_workspace_id_seq;
ALTER INDEX projects_pkey RENAME TO workspaces_pkey;
ALTER TABLE workspaces RENAME CONSTRAINT projects_created_by_fkey TO workspaces_created_by_fkey;

ALTER TABLE project_members RENAME TO workspace_members;
ALTER TABLE workspace_members RENAME COLUMN project_id TO workspace_id;
ALTER TABLE workspace_members RENAME CONSTRAINT project_members_pkey TO workspace_members_pkey;
ALTER TABLE workspace_members RENAME CONSTRAINT project_members_project_id_fkey TO workspace_members_workspace_id_fkey;
ALTER TABLE workspace_members RENAME CONSTRAINT project_members_user_id_fkey TO workspace_members_user_id_fkey;
ALTER TABLE workspace_members RENAME CONSTRAINT project_members_role_check TO workspace_members_role_check;

ALTER TABLE tracks RENAME COLUMN project_id TO workspace_id;
ALTER TABLE tracks RENAME CONSTRAINT tracks_project_id_fkey TO tracks_workspace_id_fkey;

ALTER TABLE sources RENAME COLUMN project_id TO workspace_id;
ALTER TABLE sources RENAME CONSTRAINT sources_project_id_fkey TO sources_workspace_id_fkey;
ALTER INDEX idx_sources_project_id RENAME TO idx_sources_workspace_id;

-- get_project_workspace() -> get_workspace_tracks(), body updated for the
-- renamed tracks.workspace_id column. Body copied from the CURRENT definition
-- (last redefined in 0007_graph_state_jsonb.up.sql, which dropped
-- graph_nodes/graph_edges in favor of graphs.graph_state JSONB -- NOT the
-- stale 0006 body).
DROP FUNCTION get_project_workspace(INTEGER);
CREATE FUNCTION get_workspace_tracks(p_workspace_id INTEGER)
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
    WHERE t.workspace_id = p_workspace_id
    GROUP BY t.track_id, t.name, t.extension, t.length_seconds, t.created_at
    ORDER BY t.created_at ASC;
$$;

COMMIT;
