-- Signal sources: real audio files (name + extension + duration + raw payload only --
-- no regions/trim/gain/loop-point) selectable in the Creator surface's shared "Try it"
-- preview/audition session as an alternative to the synthetic 440Hz test tone.
--
-- Disambiguation: this is unrelated to the "source"/"sink" nodeType values used inside
-- Editor-side graph_state JSONB (see migration 0008_source_sink_nodes) -- that's a
-- different, Editor-only DAG boundary-marker concept with its own tables.

BEGIN;

CREATE TABLE IF NOT EXISTS sources (
  source_id SERIAL PRIMARY KEY,
  project_id INTEGER REFERENCES projects(project_id) ON DELETE SET NULL,
  name TEXT NOT NULL,
  extension TEXT NOT NULL,
  length_seconds REAL NOT NULL CHECK (length_seconds >= 0),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_sources_project_id ON sources(project_id);

CREATE TABLE IF NOT EXISTS source_storage (
  source_id INTEGER PRIMARY KEY REFERENCES sources(source_id) ON DELETE CASCADE,
  storage_type TEXT NOT NULL DEFAULT 'inline',
  data BYTEA,
  uri TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

COMMIT;
