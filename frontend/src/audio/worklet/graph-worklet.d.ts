// Ambient type declaration for graph-worklet.js's one export meant for
// non-worklet consumption: GraphCompiler.test.ts imports
// generateTransformFunction directly to assert on its generated source
// string (see the "generateTransformFunction write-through codegen" describe
// block there). The rest of the file is loaded at runtime via
// audioWorklet.addModule(URL) (see useWorkletSetup.ts), never imported as a
// module elsewhere, so it has no other typed surface.
import type { CompiledGraph } from "../pipeline/compile-graph/compiledGraph";

export function generateTransformFunction(
  compiledGraph: CompiledGraph,
): (...args: unknown[]) => unknown;
