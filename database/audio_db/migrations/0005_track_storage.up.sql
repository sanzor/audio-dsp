BEGIN;

CREATE TABLE IF NOT EXISTS track_storage (
  track_id INTEGER PRIMARY KEY REFERENCES tracks(track_id) ON DELETE CASCADE,
  storage_type TEXT NOT NULL DEFAULT 'inline',
  data BYTEA,
  uri TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

INSERT INTO track_storage (track_id, data)
SELECT track_id, canonical_audio FROM tracks;

ALTER TABLE tracks DROP COLUMN canonical_audio;

COMMIT;
