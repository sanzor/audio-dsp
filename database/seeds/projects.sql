-- ==========================================================================
-- SEED PROJECTS (DEV)
-- ==========================================================================
-- Creates a default project owned by the admin dev user
-- and adds the test user as a viewer.
-- Safe to run multiple times.

INSERT INTO projects (name, created_by)
SELECT 'Default Project', u.user_id
FROM users u
WHERE u.email = 'admin@gmail.com'
  AND NOT EXISTS (
    SELECT 1
    FROM projects p
    WHERE p.name = 'Default Project'
  );

INSERT INTO project_members (project_id, user_id, role)
VALUES
  (
    (SELECT project_id FROM projects WHERE name = 'Default Project' ORDER BY project_id LIMIT 1),
    (SELECT user_id FROM users WHERE email = 'admin@gmail.com'),
    'owner'
  ),
  (
    (SELECT project_id FROM projects WHERE name = 'Default Project' ORDER BY project_id LIMIT 1),
    (SELECT user_id FROM users WHERE email = 'test@gmail.com'),
    'editor'
  )
ON CONFLICT (project_id, user_id) DO UPDATE
SET role = EXCLUDED.role;
