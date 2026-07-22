-- Bucket 1 (compile check): transform_resources becomes a real, immutable
-- snapshot of what a successful compile produced, not just a ticket marker.
ALTER TABLE transform_resources
  ADD COLUMN wasm_bytecode BYTEA NOT NULL DEFAULT ''::bytea,
  ADD COLUMN name TEXT NOT NULL DEFAULT '',
  ADD COLUMN description TEXT,
  ADD COLUMN ports JSONB NOT NULL DEFAULT '[]'::jsonb,
  ADD COLUMN params JSONB NOT NULL DEFAULT '[]'::jsonb;

ALTER TABLE transform_resources ALTER COLUMN wasm_bytecode DROP DEFAULT;
ALTER TABLE transform_resources ALTER COLUMN name DROP DEFAULT;
ALTER TABLE transform_resources ALTER COLUMN ports DROP DEFAULT;
ALTER TABLE transform_resources ALTER COLUMN params DROP DEFAULT;

-- Bucket 2 (save): the creator's current saved state — source always,
-- binary/metadata only when a compiled resource was attached at save time.
CREATE TABLE transform_saved_state (
  transform_id   BIGINT       PRIMARY KEY REFERENCES transforms(transform_id) ON DELETE CASCADE,
  source_code    TEXT         NOT NULL DEFAULT '',
  wasm_bytecode  BYTEA,
  name           TEXT,
  description    TEXT,
  ports          JSONB,
  params         JSONB,
  updated_at     TIMESTAMPTZ  NOT NULL DEFAULT now()
);

INSERT INTO transform_saved_state (transform_id, source_code)
SELECT transform_id, COALESCE(source_code, '') FROM transforms;

ALTER TABLE transforms DROP COLUMN source_code;

-- Bucket 3 (publish) is unchanged: transform_binaries + transforms.{name,description}
-- + transform_ports/transform_params remain the live/published artifact.

CREATE OR REPLACE FUNCTION get_transform_definition(p_transform_id BIGINT)
RETURNS TABLE (
  transform_id BIGINT,
  name TEXT,
  description TEXT,
  icon TEXT,
  source_code TEXT,
  ports JSONB,
  params JSONB
)
LANGUAGE sql
STABLE
AS $$
  SELECT
    t.transform_id,
    t.name,
    t.description,
    t.icon,
    ss.source_code,
    COALESCE(
      (
        SELECT jsonb_agg(
          jsonb_build_object(
            'port_id', tp.port_id,
            'transform_id', tp.transform_id,
            'name', tp.name,
            'direction', tp.direction,
            'port_order', tp.port_order,
            'description', tp.description
          )
          ORDER BY
            CASE WHEN tp.direction = 'input' THEN 0 ELSE 1 END,
            tp.port_order,
            tp.port_id
        )
        FROM transform_ports tp
        WHERE tp.transform_id = t.transform_id
      ),
      '[]'::jsonb
    ) AS ports,
    COALESCE(
      (
        SELECT jsonb_agg(
          jsonb_build_object(
            'param_id', tpa.param_id,
            'transform_id', tpa.transform_id,
            'name', tpa.name,
            'param_order', tpa.param_order,
            'default_value', tpa.default_value,
            'min_value', tpa.min_value,
            'max_value', tpa.max_value,
            'description', tpa.description
          )
          ORDER BY tpa.param_order, tpa.param_id
        )
        FROM transform_params tpa
        WHERE tpa.transform_id = t.transform_id
      ),
      '[]'::jsonb
    ) AS params
  FROM transforms t
  LEFT JOIN transform_saved_state ss ON ss.transform_id = t.transform_id
  WHERE t.transform_id = p_transform_id;
$$;
