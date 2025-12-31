-- PostgreSQL schema for the DAW data models.
-- The design mirrors the Rust DTOs and domain models for tracks, region sets, regions, and graphs.

BEGIN;

-- Tracks are the root aggregate. Each track stores canonical audio plus metadata.
CREATE TABLE IF NOT EXISTS tracks (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    extension TEXT NOT NULL,
    length_seconds DOUBLE PRECISION NOT NULL CHECK (length_seconds >= 0),
    canonical_audio BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_tracks_name_lower ON tracks (LOWER(name));

-- Region sets partition a track into groupings of regions.
CREATE TABLE IF NOT EXISTS region_sets (
    id UUID PRIMARY KEY,
    track_id UUID NOT NULL REFERENCES tracks(id) ON DELETE CASCADE,
    track_length DOUBLE PRECISION NOT NULL CHECK (track_length >= 0),
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (track_id, name)
);

CREATE INDEX IF NOT EXISTS idx_region_sets_track_id ON region_sets (track_id);

-- Regions capture start/end markers inside a region set.
CREATE TABLE IF NOT EXISTS regions (
    id UUID PRIMARY KEY,
    region_set_id UUID NOT NULL REFERENCES region_sets(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    start_time DOUBLE PRECISION NOT NULL CHECK (start_time >= 0),
    end_time DOUBLE PRECISION NOT NULL CHECK (end_time > start_time),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_regions_region_set_id ON regions (region_set_id);

-- Graphs (automation graphs, effect graphs, etc.) belong 1:1 to regions.
CREATE TABLE IF NOT EXISTS graphs (
    id UUID PRIMARY KEY,
    region_id UUID NOT NULL REFERENCES regions(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (region_id)
);

COMMIT;
