export interface TransformSummary {
  transform_id: number;
  name: string;
  description?: string;
  icon?: string;
}

export interface TransformParam {
  param_id: number;
  name: string;
  param_order: number;
  default_value: number;
  min_value?: number;
  max_value?: number;
  description?: string;
}

export interface TransformPort {
  port_id: number;
  name: string;
  direction: "input" | "output";
  port_order: number;
  description?: string;
}

export interface TransformDefinition extends TransformSummary {
  ports: TransformPort[];
  params: TransformParam[];
}
