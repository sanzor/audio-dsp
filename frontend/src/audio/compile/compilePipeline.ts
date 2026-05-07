import type { ActiveGraph } from "@/Stores/ActiveGraphState"
import type { CompileOutput, CompiledGraph } from "@/audio/types/compiled"
import { compile } from "./graph-compiler"

type BinaryStatus = "fetching" | "ready" | "error"

export type CompileResult =
  | { status: "no-transforms" }
  | { status: "binaries-missing"; missingIds: number[] }
  | { status: "binaries-failed"; failedIds: number[] }
  | { status: "error"; message: string }
  | { status: "ok"; compiled: CompiledGraph }

export function compilePipeline(
  graph: ActiveGraph | null,
  transformBinariesMap: Map<number, Uint8Array>,
  transformBinaryStatusMap: Map<number, BinaryStatus>,
): CompileResult {
  if (!graph || graph.nodes.size === 0) {
    return { status: "no-transforms" }
  }

  // Step 1: pure instruction scheduling — produces transformIds without needing binaries yet.
  let output: CompileOutput
  try {
    output = compile(graph)
  } catch (error) {
    return { status: "error", message: error instanceof Error ? error.message : "Compile failed." }
  }

  const failedIds = output.transformIds.filter((id) => transformBinaryStatusMap.get(id) === "error")
  if (failedIds.length > 0) {
    return { status: "binaries-failed", failedIds }
  }

  const missingIds = output.transformIds.filter((id) => !transformBinariesMap.has(id))
  if (missingIds.length > 0) {
    return { status: "binaries-missing", missingIds }
  }

  // Step 2: map instruction schedule + binaries → worklet payload.
  const compiled = mapBinaries(output, transformBinariesMap)
  if (!compiled) {
    return { status: "error", message: "Binary attachment failed unexpectedly." }
  }

  return { status: "ok", compiled }
}

export function mapBinaries(
  output: CompileOutput,
  binaries: Map<number, Uint8Array>,
): CompiledGraph | null {
  const wasmBuffers: ArrayBuffer[] = []
  for (const transformId of output.transformIds) {
    const binary = binaries.get(transformId)
    if (!binary) return null
    wasmBuffers.push(binary.slice().buffer)
  }

  return {
    instructions: output.instructions,
    bufCount: output.bufCount,
    feedbackCount: output.feedbackCount,
    wasmBuffers,
    paramsByInstance: output.paramsByInstance,
  }
}
