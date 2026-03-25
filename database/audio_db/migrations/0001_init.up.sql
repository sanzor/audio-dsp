-- Initial schema for Audio DSP domain models.
-- Targets Postgres.

BEGIN;

CREATE TABLE IF NOT EXISTS tracks (
  track_id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  extension TEXT NOT NULL,
  length_seconds REAL NOT NULL CHECK (length_seconds >= 0),
  canonical_audio BYTEA NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS region_sets (
  region_set_id SERIAL PRIMARY KEY,
  track_id INTEGER NOT NULL REFERENCES tracks(track_id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  track_length_seconds REAL NOT NULL CHECK (track_length_seconds >= 0),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_region_sets_track_id ON region_sets(track_id);

CREATE TABLE IF NOT EXISTS regions (
  region_id SERIAL PRIMARY KEY,
  region_set_id INTEGER NOT NULL REFERENCES region_sets(region_set_id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  start_time_seconds REAL NOT NULL CHECK (start_time_seconds >= 0),
  end_time_seconds REAL NOT NULL CHECK (end_time_seconds >= 0),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT regions_time_order CHECK (end_time_seconds >= start_time_seconds)
);

CREATE INDEX IF NOT EXISTS idx_regions_region_set_id ON regions(region_set_id);

CREATE TABLE IF NOT EXISTS graphs (
  graph_id SERIAL PRIMARY KEY,
  region_id INTEGER UNIQUE REFERENCES regions(region_id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_graphs_region_id ON graphs(region_id);

CREATE TABLE IF NOT EXISTS graph_nodes (
  node_id SERIAL PRIMARY KEY,
  graph_id INTEGER NOT NULL REFERENCES graphs(graph_id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_graph_nodes_graph_id ON graph_nodes(graph_id);

CREATE TABLE IF NOT EXISTS graph_edges (
  edge_id SERIAL PRIMARY KEY,
  graph_id INTEGER NOT NULL REFERENCES graphs(graph_id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_graph_edges_graph_id ON graph_edges(graph_id);

COMMIT;
