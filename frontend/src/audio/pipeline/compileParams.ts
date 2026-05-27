import type { ActiveGraph } from "@/Stores/ActiveGraphState";

export interface CompileParams {
  graph: ActiveGraph;
  transformIds: number[];
  binaries: Map<number, Uint8Array>;
}