BEGIN;

ALTER TABLE transform ADD COLUMN owner_user_id INTEGER REFERENCES users(user_id);

-- TODO(owner-backfill): young/dev-stage schema -- existing transforms predate
-- per-user ownership, so there's no real historical owner to recover. Back
-- fill to the first admin (or, failing that, the first user) rather than
-- leaving a NULL model. Revisit only if real ownership history is ever
-- needed for pre-existing rows.
UPDATE transform SET owner_user_id = (SELECT user_id FROM users WHERE is_admin = true ORDER BY user_id ASC LIMIT 1)
WHERE owner_user_id IS NULL;
UPDATE transform SET owner_user_id = (SELECT user_id FROM users ORDER BY user_id ASC LIMIT 1)
WHERE owner_user_id IS NULL;

ALTER TABLE transform ALTER COLUMN owner_user_id SET NOT NULL;
CREATE INDEX idx_transform_owner_user_id ON transform(owner_user_id);

COMMIT;
