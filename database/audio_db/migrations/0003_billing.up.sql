BEGIN;

CREATE TABLE IF NOT EXISTS tier_configs (
  id                     SERIAL PRIMARY KEY,
  tier                   TEXT UNIQUE NOT NULL,
  max_projects           INTEGER NOT NULL,
  max_tracks_per_project INTEGER NOT NULL,
  max_storage_bytes      BIGINT NOT NULL,
  updated_at             TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS products (
  id          SERIAL PRIMARY KEY,
  name        TEXT NOT NULL,
  description TEXT,
  tier        TEXT NOT NULL,
  price_cents BIGINT NOT NULL,
  currency    TEXT NOT NULL DEFAULT 'usd',
  is_active   BOOLEAN NOT NULL DEFAULT true,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS subscriptions (
  id         SERIAL PRIMARY KEY,
  user_id    INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
  tier       TEXT NOT NULL,
  is_active  BOOLEAN NOT NULL DEFAULT true,
  started_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  expires_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS purchased_products (
  id           SERIAL PRIMARY KEY,
  user_id      INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
  product_id   INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
  purchased_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS invoices (
  id                SERIAL PRIMARY KEY,
  user_id           INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
  stripe_invoice_id TEXT NOT NULL,
  amount            BIGINT NOT NULL,
  currency          TEXT NOT NULL,
  status            TEXT NOT NULL,
  hosted_url        TEXT,
  created_at        TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS usage (
  id                  SERIAL PRIMARY KEY,
  user_id             INTEGER NOT NULL UNIQUE REFERENCES users(user_id) ON DELETE CASCADE,
  project_count       BIGINT NOT NULL DEFAULT 0,
  total_track_count   BIGINT NOT NULL DEFAULT 0,
  total_storage_bytes BIGINT NOT NULL DEFAULT 0,
  updated_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

COMMIT;
