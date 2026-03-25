-- ==========================================================================
-- SEED PRODUCTS
-- ==========================================================================
-- Safe to run multiple times.

INSERT INTO products (name, description, tier, price_cents, currency, is_active)
VALUES
    ('Starter',    'Up to 20 projects',   'Standard',  500,   'usd', true),
    ('Growth',     'Up to 100 projects',  'Premium',   2000,  'usd', true),
    ('Enterprise', 'Unlimited projects',  'BankLevel', 30000, 'usd', true)
ON CONFLICT DO NOTHING;
