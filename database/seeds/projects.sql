-- ==========================================================================
-- SEED WORKSPACES (DEV)
-- ==========================================================================
-- Creates a default workspace owned by the admin dev user
-- and several test-account workspaces with seeded audio-ready memberships.
-- Safe to run multiple times.

INSERT INTO workspaces (name, created_by)
SELECT 'Default Project', u.user_id
FROM users u
WHERE u.email = 'admin@gmail.com'
  AND NOT EXISTS (
    SELECT 1
    FROM workspaces p
    WHERE p.name = 'Default Project'
  );

INSERT INTO workspaces (name, created_by)
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
    FROM workspaces p
    WHERE p.name = seed.name
  );

INSERT INTO workspace_members (workspace_id, user_id, role)
VALUES
  (
    (SELECT workspace_id FROM workspaces WHERE name = 'Default Project' ORDER BY workspace_id LIMIT 1),
    (SELECT user_id FROM users WHERE email = 'admin@gmail.com'),
    'owner'
  ),
  (
    (SELECT workspace_id FROM workspaces WHERE name = 'Default Project' ORDER BY workspace_id LIMIT 1),
    (SELECT user_id FROM users WHERE email = 'test@gmail.com'),
    'editor'
  ),
  (
    (SELECT workspace_id FROM workspaces WHERE name = 'Canonical Audio Lab' ORDER BY workspace_id LIMIT 1),
    (SELECT user_id FROM users WHERE email = 'test@gmail.com'),
    'owner'
  ),
  (
    (SELECT workspace_id FROM workspaces WHERE name = 'Canonical Audio Lab' ORDER BY workspace_id LIMIT 1),
    (SELECT user_id FROM users WHERE email = 'admin@gmail.com'),
    'viewer'
  ),
  (
    (SELECT workspace_id FROM workspaces WHERE name = 'Mix Review Sandbox' ORDER BY workspace_id LIMIT 1),
    (SELECT user_id FROM users WHERE email = 'test@gmail.com'),
    'owner'
  ),
  (
    (SELECT workspace_id FROM workspaces WHERE name = 'Mix Review Sandbox' ORDER BY workspace_id LIMIT 1),
    (SELECT user_id FROM users WHERE email = 'admin@gmail.com'),
    'viewer'
  ),
  (
    (SELECT workspace_id FROM workspaces WHERE name = 'Regression Pack' ORDER BY workspace_id LIMIT 1),
    (SELECT user_id FROM users WHERE email = 'test@gmail.com'),
    'owner'
  ),
  (
    (SELECT workspace_id FROM workspaces WHERE name = 'Regression Pack' ORDER BY workspace_id LIMIT 1),
    (SELECT user_id FROM users WHERE email = 'admin@gmail.com'),
    'viewer'
  )
ON CONFLICT (workspace_id, user_id) DO UPDATE
SET role = EXCLUDED.role;
