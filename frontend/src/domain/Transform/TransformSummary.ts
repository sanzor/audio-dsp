export interface TransformSummary {
  transform_id: number;
  name: string;
  description?: string;
  icon?: string;
  kind: "primitive" | "composite";
}
