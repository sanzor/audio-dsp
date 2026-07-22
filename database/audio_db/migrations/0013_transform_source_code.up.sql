ALTER TABLE transforms ADD COLUMN source_code TEXT;

-- Postgres disallows CREATE OR REPLACE FUNCTION when the RETURNS TABLE
-- column list changes (migration 0010 defined this function with 6 output
-- columns; here it grows to 7 with source_code added) — drop first.
DROP FUNCTION IF EXISTS get_transform_definition(BIGINT);

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
    t.source_code,
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
  WHERE t.transform_id = p_transform_id;
$$;
