export interface TransformParam {
  param_id: number;
  name: string;
  param_order: number;
  default_value: number;
  min_value?: number;
  max_value?: number;
  description?: string;
}
