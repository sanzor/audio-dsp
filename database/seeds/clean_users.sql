-- ==========================================================================
-- CLEAN USERS (DEV)
-- ==========================================================================
-- Removes all users and their projects/memberships and resets serial IDs
-- so subsequent seed runs recreate deterministic IDs.
-- Safe to run multiple times.

BEGIN;

TRUNCATE TABLE
  users,
  projects,
  tracks
RESTART IDENTITY CASCADE;

COMMIT;
