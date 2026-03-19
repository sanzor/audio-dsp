-- ==========================================================================
-- SEED TIER CONFIGS
-- ==========================================================================
-- window_size_secs: 2592000 = 30 days
-- Safe to run multiple times (upsert).

INSERT INTO tier_configs (tier, token_limit, window_size_secs)
VALUES
    ('free',        10000,       2592000),
    ('premium',     10000000,    2592000),
    ('bank_level',  100000000,   2592000)
ON CONFLICT (tier) DO UPDATE
    SET token_limit      = EXCLUDED.token_limit,
        window_size_secs = EXCLUDED.window_size_secs;
