-- Multi-input named ports: transform_ports gains a port kind (Program vs.
-- Sidechain — determines unwired-port behavior at the routing layer, not
-- enforced here) and a cardinality (Single vs. Many — how many edges may
-- land on the port). Both NOT NULL DEFAULT so every currently-published
-- transform is correctly backfilled by the default alone: every existing
-- port predates this feature and was implicitly a main, single-edge port.
-- Same pattern as migrations 0014/0015. See
-- agents/decisions/0004-multi-input-named-ports.md.
ALTER TABLE transform_ports
  ADD COLUMN kind TEXT NOT NULL DEFAULT 'program' CHECK (kind IN ('program', 'sidechain')),
  ADD COLUMN cardinality TEXT NOT NULL DEFAULT 'single' CHECK (cardinality IN ('single', 'many'));

-- Drop the defaults once the backfill above has run — same pattern as
-- migration 0014's transform_resources.ports/params columns. Every future
-- write goes through publish_compiled_transform, which always supplies both
-- values explicitly (introspected from compiled metadata); a write that
-- forgets to would now fail loudly instead of silently defaulting.
ALTER TABLE transform_ports ALTER COLUMN kind DROP DEFAULT;
ALTER TABLE transform_ports ALTER COLUMN cardinality DROP DEFAULT;

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
            'description', tp.description,
            'kind', tp.kind,
            'cardinality', tp.cardinality
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
