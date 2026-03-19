-- ==========================================================================
-- SEED PROJECTS (DEV)
-- ==========================================================================
-- Creates a default project owned by the admin dev user
-- and adds the test user as a viewer.
-- Safe to run multiple times.

INSERT INTO projects (name, created_by)
VALUES ('Default Project', (SELECT user_id FROM users WHERE email = 'admin@gmail.com'))
ON CONFLICT DO NOTHING;

INSERT INTO project_members (project_id, user_id, role)
VALUES
  (
    (SELECT project_id FROM projects WHERE name = 'Default Project'),
    (SELECT user_id FROM users WHERE email = 'admin@gmail.com'),
    'owner'
  ),
  (
    (SELECT project_id FROM projects WHERE name = 'Default Project'),
    (SELECT user_id FROM users WHERE email = 'test@gmail.com'),
    'viewer'
  )
ON CONFLICT (project_id, user_id) DO UPDATE
SET role = EXCLUDED.role;
