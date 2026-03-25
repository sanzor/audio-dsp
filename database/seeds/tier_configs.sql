-- ==========================================================================
-- SEED TIER CONFIGS
-- ==========================================================================
-- Safe to run multiple times (upsert).

INSERT INTO tier_configs (tier, max_projects, max_tracks_per_project, max_storage_bytes)
VALUES
    ('Free',       3,    50,   1073741824),
    ('Standard',   20,   500,  10737418240),
    ('Premium',    100,  5000, 107374182400),
    ('BankLevel',  -1,   -1,   -1)
ON CONFLICT (tier) DO UPDATE
    SET max_projects           = EXCLUDED.max_projects,
        max_tracks_per_project = EXCLUDED.max_tracks_per_project,
        max_storage_bytes      = EXCLUDED.max_storage_bytes,
        updated_at             = now();
