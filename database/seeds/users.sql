-- ==========================================================================
-- SEED USERS (DEV)
-- ==========================================================================
-- Safe to run multiple times.

INSERT INTO users (user_id, email, name, picture, is_admin, is_active, is_verified, password_hash)
VALUES
  ('01JSEED0000000000000000001', 'admin@gmail.com',                  'Admin',                  '', true,  true, true, 'admin'),
  ('01JSEED0000000000000000002', 'test@gmail.com',                   'Test User',              '', false, true, true, 'test'),
  ('01JSEED0000000000000000003', 'bercovici.adrian.simon@gmail.com', 'Adrian Simon Bercovici', '', true,  true, true, 'admin')
ON CONFLICT (email) DO UPDATE
SET name          = EXCLUDED.name,
    is_admin      = EXCLUDED.is_admin,
    is_active     = EXCLUDED.is_active,
    is_verified   = EXCLUDED.is_verified,
    password_hash = EXCLUDED.password_hash;
