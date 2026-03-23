export interface TransformPort {
  port_id: number;
  name: string;
  direction: "input" | "output";
  port_order: number;
  description?: string;
}

export interface Transform {
  transform_id: number;
  name: string;
  description?: string;
  icon?: string;
  ports: TransformPort[];
}
