-- ==========================================================================
-- CLEAN USERS (DEV)
-- ==========================================================================
-- Removes all users and their workspaces/memberships and resets serial IDs
-- so subsequent seed runs recreate deterministic IDs.
-- Safe to run multiple times.

BEGIN;

TRUNCATE TABLE
  users,
  workspaces,
  tracks
RESTART IDENTITY CASCADE;

COMMIT;
