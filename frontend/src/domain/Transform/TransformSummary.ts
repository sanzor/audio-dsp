export interface TransformSummary {
  transform_id: number;
  name: string;
  description?: string;
  icon?: string;
  kind: "primitive" | "composite";
  // Live in transform_binary (primitive) or transform_composite (composite).
  published: boolean;
}
