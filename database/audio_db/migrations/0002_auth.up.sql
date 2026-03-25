BEGIN;

CREATE TABLE IF NOT EXISTS users (
  user_id       SERIAL PRIMARY KEY,
  email         TEXT UNIQUE NOT NULL,
  name          TEXT NOT NULL DEFAULT '',
  picture       TEXT NOT NULL DEFAULT '',
  google_sub_id TEXT UNIQUE,
  is_admin      BOOLEAN NOT NULL DEFAULT false,
  is_active     BOOLEAN NOT NULL DEFAULT true,
  created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS projects (
  project_id  SERIAL PRIMARY KEY,
  name        TEXT NOT NULL,
  created_by  INTEGER NOT NULL REFERENCES users(user_id),
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- One role per user per project: owner | editor | viewer
CREATE TABLE IF NOT EXISTS project_members (
  project_id  INTEGER NOT NULL REFERENCES projects(project_id) ON DELETE CASCADE,
  user_id     INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
  role        TEXT NOT NULL DEFAULT 'viewer' CHECK (role IN ('owner', 'editor', 'viewer')),
  joined_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
  PRIMARY KEY (project_id, user_id)
);

ALTER TABLE tracks ADD COLUMN IF NOT EXISTS project_id INTEGER REFERENCES projects(project_id) ON DELETE SET NULL;

ALTER TABLE users ADD COLUMN IF NOT EXISTS password_hash TEXT;
ALTER TABLE users ADD COLUMN IF NOT EXISTS is_verified BOOLEAN NOT NULL DEFAULT false;

COMMIT;
