BEGIN;

CREATE TABLE transform_grants (
  grant_id             BIGSERIAL    PRIMARY KEY,
  transform_id         BIGINT       NOT NULL REFERENCES transform(transform_id) ON DELETE CASCADE,
  grantee_user_id      INTEGER      REFERENCES users(user_id) ON DELETE CASCADE,
  grantee_workspace_id INTEGER      REFERENCES workspaces(workspace_id) ON DELETE CASCADE,
  granted_by           INTEGER      NOT NULL REFERENCES users(user_id),
  created_at           TIMESTAMPTZ  NOT NULL DEFAULT now(),
  CONSTRAINT transform_grants_exactly_one_grantee
    CHECK (num_nonnulls(grantee_user_id, grantee_workspace_id) = 1)
);

-- Plain UNIQUE(transform_id, grantee_user_id) would not stop duplicate
-- workspace-only grants -- Postgres treats every NULL as distinct for
-- uniqueness, and grantee_user_id is NULL on every workspace-grant row.
-- Partial indexes scoped to the non-null case are required instead.
CREATE UNIQUE INDEX idx_transform_grants_unique_user
  ON transform_grants(transform_id, grantee_user_id)
  WHERE grantee_user_id IS NOT NULL;

CREATE UNIQUE INDEX idx_transform_grants_unique_workspace
  ON transform_grants(transform_id, grantee_workspace_id)
  WHERE grantee_workspace_id IS NOT NULL;

CREATE INDEX idx_transform_grants_transform_id ON transform_grants(transform_id);
CREATE INDEX idx_transform_grants_grantee_user_id ON transform_grants(grantee_user_id) WHERE grantee_user_id IS NOT NULL;
CREATE INDEX idx_transform_grants_grantee_workspace_id ON transform_grants(grantee_workspace_id) WHERE grantee_workspace_id IS NOT NULL;

COMMIT;
