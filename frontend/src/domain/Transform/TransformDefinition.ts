import type { TransformParam } from "./TransformParam";
import type { TransformPort } from "./TransformPort";
import type { TransformSummary } from "./TransformSummary";



export interface TransformDefinition extends TransformSummary {
  ports: TransformPort[];
  params: TransformParam[];
}
