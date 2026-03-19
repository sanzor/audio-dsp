-- ==========================================================================
-- SEED PRODUCTS
-- ==========================================================================
-- Safe to run multiple times.

INSERT INTO products (name, description, project_amount, price_cents, currency, is_active)
VALUES
    ('Starter Pack',    '5 projects',    5,    500,   'usd', true),
    ('Growth Pack',     '25 projects',   25,   2000,  'usd', true),
    ('Business Pack',   '100 projects',  100,  7000,  'usd', true),
    ('Enterprise Pack', '500 projects',  500,  30000, 'usd', true)
ON CONFLICT DO NOTHING;
