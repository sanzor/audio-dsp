-- ==========================================================================
-- SEED PROJECTS (DEV)
-- ==========================================================================
-- Creates a default project owned by the admin dev user
-- and adds the test user as a viewer.
-- Safe to run multiple times.

INSERT INTO projects (project_id, name, created_by)
VALUES ('01JSEED000000000000000P001', 'Default Project', '01JSEED0000000000000000001')
ON CONFLICT (project_id) DO UPDATE
SET name = EXCLUDED.name;

INSERT INTO project_members (project_id, user_id, role)
VALUES
  ('01JSEED000000000000000P001', '01JSEED0000000000000000001', 'owner'),
  ('01JSEED000000000000000P001', '01JSEED0000000000000000002', 'viewer')
ON CONFLICT (project_id, user_id) DO UPDATE
SET role = EXCLUDED.role;
