import type { TransformParam } from "./TransformParam";
import type { TransformPort } from "./TransformPort";
import type { TransformSummary } from "./TransformSummary";



export interface TransformDefinition extends TransformSummary {
  source_code?: string;
  ports: TransformPort[];
  params: TransformParam[];
}
