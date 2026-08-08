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
  // Composite-only sub-state between Save and Publish: true once the
  // currently-persisted graph_definition has passed the explicit validate
  // action (POST /transforms/{id}/validate). Any subsequent save flips this
  // back to false. Always false for primitives. See
  // agents/decisions/0007-composite-draft-validation-gate.md.
  is_validated: boolean;
}
