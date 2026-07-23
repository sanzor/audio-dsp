
export interface TransformPort {
  port_id: number;
  name: string;
  direction: "input" | "output";
  port_order: number;
  description?: string;
  // Program = main signal (fails closed if unwired); Sidechain = control/
  // detector signal (unwired always resolves to silence). See
  // agents/transforms.md's ABI contract section.
  kind: "program" | "sidechain";
  // Single = exactly one edge may target this port; Many = N edges sum into
  // it (opt-in, e.g. multiple mic feeds into one "mix" port).
  cardinality: "single" | "many";
}