import type { TransformParam } from "./TransformParam";
import type { TransformPort } from "./TransformPort";
import type { TransformSummary } from "./TransformSummary";
import type { CompositeGraphDefinition } from "./CompositeGraphDefinition";



export interface TransformDefinition extends TransformSummary {
  source_code?: string;
  // Present only for kind = "composite".
  graph_definition?: CompositeGraphDefinition;
  ports: TransformPort[];
  params: TransformParam[];
}
