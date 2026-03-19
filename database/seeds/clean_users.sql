-- ==========================================================================
-- CLEAN USERS (DEV)
-- ==========================================================================
-- Removes all users and their projects/memberships.
-- Safe to run multiple times.

BEGIN;

DELETE FROM project_members;
DELETE FROM projects;
DELETE FROM users;

COMMIT;
