CREATE UNIQUE INDEX IF NOT EXISTS idx_transforms_name_unique ON transforms (name);

CREATE TABLE transform_params (
  param_id       BIGSERIAL    PRIMARY KEY,
  transform_id   BIGINT       NOT NULL REFERENCES transforms(transform_id) ON DELETE CASCADE,
  name           TEXT         NOT NULL,
  param_order    INT          NOT NULL DEFAULT 0,
  default_value  REAL         NOT NULL,
  min_value      REAL,
  max_value      REAL,
  description    TEXT,
  created_at     TIMESTAMPTZ  NOT NULL DEFAULT now(),
  UNIQUE (transform_id, name),
  UNIQUE (transform_id, param_order)
);

CREATE INDEX idx_transform_params_transform_id ON transform_params(transform_id);

CREATE TABLE transform_binaries (
  transform_id   BIGINT       PRIMARY KEY REFERENCES transforms(transform_id) ON DELETE CASCADE,
  wasm_bytecode  BYTEA        NOT NULL,
  source         TEXT,
  created_at     TIMESTAMPTZ  NOT NULL DEFAULT now(),
  updated_at     TIMESTAMPTZ  NOT NULL DEFAULT now()
);
