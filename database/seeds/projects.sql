-- ==========================================================================
-- SEED PROJECTS (DEV)
-- ==========================================================================
-- Creates a default project owned by the admin dev user
-- and several test-account projects with seeded audio-ready memberships.
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

INSERT INTO projects (name, created_by)
SELECT seed.name, u.user_id
FROM users u
CROSS JOIN (
  VALUES
    ('Canonical Audio Lab'),
    ('Mix Review Sandbox'),
    ('Regression Pack')
) AS seed(name)
WHERE u.email = 'test@gmail.com'
  AND NOT EXISTS (
    SELECT 1
    FROM projects p
    WHERE p.name = seed.name
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
  ),
  (
    (SELECT project_id FROM projects WHERE name = 'Canonical Audio Lab' ORDER BY project_id LIMIT 1),
    (SELECT user_id FROM users WHERE email = 'test@gmail.com'),
    'owner'
  ),
  (
    (SELECT project_id FROM projects WHERE name = 'Canonical Audio Lab' ORDER BY project_id LIMIT 1),
    (SELECT user_id FROM users WHERE email = 'admin@gmail.com'),
    'viewer'
  ),
  (
    (SELECT project_id FROM projects WHERE name = 'Mix Review Sandbox' ORDER BY project_id LIMIT 1),
    (SELECT user_id FROM users WHERE email = 'test@gmail.com'),
    'owner'
  ),
  (
    (SELECT project_id FROM projects WHERE name = 'Mix Review Sandbox' ORDER BY project_id LIMIT 1),
    (SELECT user_id FROM users WHERE email = 'admin@gmail.com'),
    'viewer'
  ),
  (
    (SELECT project_id FROM projects WHERE name = 'Regression Pack' ORDER BY project_id LIMIT 1),
    (SELECT user_id FROM users WHERE email = 'test@gmail.com'),
    'owner'
  ),
  (
    (SELECT project_id FROM projects WHERE name = 'Regression Pack' ORDER BY project_id LIMIT 1),
    (SELECT user_id FROM users WHERE email = 'admin@gmail.com'),
    'viewer'
  )
ON CONFLICT (project_id, user_id) DO UPDATE
SET role = EXCLUDED.role;
