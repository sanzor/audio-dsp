import { useCallback } from "react"
import { useWasmBinaryStore } from "@/Stores/WasmBinaryStore"
import type { ActiveGraph } from "@/Stores/ActiveGraphState"
import { useEnsureTransformBinaries } from "@/hooks/transforms/useEnsureTransformBinaries"
import { PipelineOrchestrator } from "@/audio/orchestration/PipelineProvider"
import { AudioEffectsCompiledGraphProvider } from "@/audio/orchestration/CompiledGraphProvider"
import { AudioEffectsRuntimeStateProvider } from "@/audio/orchestration/RuntimeStateProvider"

const orchestrator = new PipelineOrchestrator(
  new AudioEffectsCompiledGraphProvider(),
  new AudioEffectsRuntimeStateProvider(),
)

export function useCompile(runtimeGraph: ActiveGraph | null): () => void {
  const ensureTransformBinaries = useEnsureTransformBinaries()

  return useCallback(() => {
    void (async () => {
      if (runtimeGraph) {
        const requiredTransformIds = Array.from(
          new Set(Array.from(runtimeGraph.nodes.values(), (node) => node.transformId))
        )
        try {
          await ensureTransformBinaries(requiredTransformIds)
        } catch {}
      }

      const { binaries, status } = useWasmBinaryStore.getState()
      const nextBinaries = new Map(binaries)
      const nextBinaryStatus = new Map(status)
      orchestrator.apply({ graph: runtimeGraph, binaries: nextBinaries, binaryStatus: nextBinaryStatus })
    })()
  }, [runtimeGraph, ensureTransformBinaries])
}
