BEGIN;

ALTER TABLE tracks ADD COLUMN canonical_audio BYTEA;

UPDATE tracks t
SET canonical_audio = ts.data
FROM track_storage ts
WHERE ts.track_id = t.track_id;

ALTER TABLE tracks ALTER COLUMN canonical_audio SET NOT NULL;

DROP TABLE track_storage;

COMMIT;
