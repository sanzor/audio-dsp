CREATE TABLE transforms (
  transform_id  BIGSERIAL    PRIMARY KEY,
  name          TEXT         NOT NULL,
  description   TEXT,
  icon          TEXT,
  created_at    TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE TABLE transform_ports (
  port_id       BIGSERIAL  PRIMARY KEY,
  transform_id  BIGINT     NOT NULL REFERENCES transforms(transform_id) ON DELETE CASCADE,
  name          TEXT       NOT NULL,
  direction     TEXT       NOT NULL CHECK (direction IN ('input', 'output')),
  port_order    INT        NOT NULL DEFAULT 0,
  description   TEXT
);

CREATE INDEX idx_transform_ports_transform_id ON transform_ports(transform_id);
