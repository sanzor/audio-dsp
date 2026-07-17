
export interface TransformPort {
  port_id: number;
  name: string;
  direction: "input" | "output";
  port_order: number;
  description?: string;
}