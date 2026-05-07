import type { ActiveGraph } from "@/Stores/ActiveGraphState"
import { compilePipeline, type CompileResult } from "@/audio/compile/compilePipeline"
import type { CompiledGraphProvider } from "./CompiledGraphProvider"
import type { RuntimeStateProvider } from "./RuntimeStateProvider"

type BinaryStatus = "fetching" | "ready" | "error"

export type CompileInput = {
  graph: ActiveGraph | null
  binaries: Map<number, Uint8Array>
  binaryStatus: Map<number, BinaryStatus>
}

export class PipelineOrchestrator {
  constructor(
    private readonly graphStore: CompiledGraphProvider,
    private readonly runtimeStateStore: RuntimeStateProvider,
  ) {}

  apply(input: CompileInput): void {
    const result = compilePipeline(input.graph, input.binaries, input.binaryStatus)
    this.dispatch(result)
  }

  private dispatch(result: CompileResult): void {
    switch (result.status) {
      case "no-transforms":
        this.graphStore.setCompiledGraph(null)
        this.runtimeStateStore.setRuntimeState("idle", "No transform nodes.")
        break

      case "binaries-missing":
        this.graphStore.setCompiledGraph(null)
        this.runtimeStateStore.setRuntimeState("hydrating", `Waiting on binaries: ${result.missingIds.join(", ")}`)
        break

      case "binaries-failed":
        this.graphStore.setCompiledGraph(null)
        this.runtimeStateStore.setRuntimeState("error", `Binary load failed: ${result.failedIds.join(", ")}`)
        break

      case "error":
        this.graphStore.setCompiledGraph(null)
        this.runtimeStateStore.setRuntimeState("error", result.message)
        break

      case "ok":
        this.graphStore.setCompiledGraph(result.compiled)
        this.runtimeStateStore.setRuntimeState("idle", null)
        break
    }
  }
}
