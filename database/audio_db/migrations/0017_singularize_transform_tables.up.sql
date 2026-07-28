-- Singularize every transform-related table name for naming consistency, and
-- rename transform_saved_state -> transform_draft (bucket 2 is the creator's
-- "draft", the name it's already called throughout agents/transforms.md and
-- the decision docs — the table name never matched the concept name).
-- ALTER TABLE RENAME preserves the table's OID, so all existing FKs, CHECK
-- constraints, and the ON DELETE CASCADE relationships keep working unchanged.
ALTER TABLE transforms RENAME TO transform;
ALTER TABLE transform_ports RENAME TO transform_port;
ALTER TABLE transform_params RENAME TO transform_param;
ALTER TABLE transform_binaries RENAME TO transform_binary;
ALTER TABLE transform_tickets RENAME TO transform_ticket;
ALTER TABLE transform_resources RENAME TO transform_resource;
ALTER TABLE transform_saved_state RENAME TO transform_draft;

-- Indexes aren't renamed automatically by ALTER TABLE RENAME — bring them in
-- line too.
ALTER INDEX idx_transforms_name_unique RENAME TO idx_transform_name_unique;
ALTER INDEX idx_transform_ports_transform_id RENAME TO idx_transform_port_transform_id;
ALTER INDEX idx_transform_params_transform_id RENAME TO idx_transform_param_transform_id;
ALTER INDEX idx_transform_tickets_transform_id RENAME TO idx_transform_ticket_transform_id;
ALTER INDEX idx_transform_tickets_issued_by RENAME TO idx_transform_ticket_issued_by;
ALTER INDEX idx_transform_tickets_status RENAME TO idx_transform_ticket_status;
ALTER INDEX idx_transform_resources_ticket_id RENAME TO idx_transform_resource_ticket_id;

-- get_transform_definition()'s body references the renamed tables; signature
-- is unchanged so a plain CREATE OR REPLACE is enough (no DROP needed).
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
        FROM transform_port tp
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
        FROM transform_param tpa
        WHERE tpa.transform_id = t.transform_id
      ),
      '[]'::jsonb
    ) AS params
  FROM transform t
  LEFT JOIN transform_draft ss ON ss.transform_id = t.transform_id
  WHERE t.transform_id = p_transform_id;
$$;
