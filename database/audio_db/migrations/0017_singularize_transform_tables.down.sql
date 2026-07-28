ALTER INDEX idx_transform_name_unique RENAME TO idx_transforms_name_unique;
ALTER INDEX idx_transform_port_transform_id RENAME TO idx_transform_ports_transform_id;
ALTER INDEX idx_transform_param_transform_id RENAME TO idx_transform_params_transform_id;
ALTER INDEX idx_transform_ticket_transform_id RENAME TO idx_transform_tickets_transform_id;
ALTER INDEX idx_transform_ticket_issued_by RENAME TO idx_transform_tickets_issued_by;
ALTER INDEX idx_transform_ticket_status RENAME TO idx_transform_tickets_status;
ALTER INDEX idx_transform_resource_ticket_id RENAME TO idx_transform_resources_ticket_id;

ALTER TABLE transform RENAME TO transforms;
ALTER TABLE transform_port RENAME TO transform_ports;
ALTER TABLE transform_param RENAME TO transform_params;
ALTER TABLE transform_binary RENAME TO transform_binaries;
ALTER TABLE transform_ticket RENAME TO transform_tickets;
ALTER TABLE transform_resource RENAME TO transform_resources;
ALTER TABLE transform_draft RENAME TO transform_saved_state;

-- Restore migration 0016's version of the function (pre-rename table names).
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
