import { compileGraph as compileActiveGraph } from "@/audio/pipeline/GraphCompiler";
import type { ActiveGraph } from "@/Stores/ActiveGraphState";
import type { CompiledGraph } from "./compiledGraph";

export interface CompiledGraphResult {
  compiledGraph: CompiledGraph;
  resolved_binaries: Record<number, Uint8Array>;
}

export function compileGraph(graph: ActiveGraph): CompiledGraph | null {
  try {
    return compileActiveGraph(graph) ?? null
  } catch {
    return null
  }
}
