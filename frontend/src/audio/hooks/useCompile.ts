import { useCallback } from "react"
import { useWasmBinaryStore } from "@/Stores/WasmBinaryStore"
import { useAudioEffectsStore } from "@/Stores/AudioEffectsStore"
import type { ActiveGraph } from "@/Stores/ActiveGraphState"
import { useEnsureTransformBinaries } from "@/hooks/transforms/useEnsureTransformBinaries"
import { compileActiveGraph } from "@/audio/pipeline/GraphCompiler"

export function useCompile(runtimeGraph: ActiveGraph | null): () => void {
  const ensureTransformBinaries = useEnsureTransformBinaries()
  const setCompiledGraph = useAudioEffectsStore((s) => s.setCompiledGraph)
  const setRuntimeState = useAudioEffectsStore((s) => s.setRuntimeState)

  return useCallback(() => {
    void (async () => {
      if (!runtimeGraph || runtimeGraph.nodes.size === 0) {
        setCompiledGraph(null)
        setRuntimeState("idle", "No transform nodes.")
        return
      }

      const requiredTransformIds = Array.from(
        new Set(Array.from(runtimeGraph.nodes.values(), (node) => node.transformId))
      )

      try {
        await ensureTransformBinaries(requiredTransformIds)
      } catch {}

      const { binaries, status } = useWasmBinaryStore.getState()
      const failedIds = requiredTransformIds.filter((id) => status.get(id) === "error")
      if (failedIds.length > 0) {
        setCompiledGraph(null)
        setRuntimeState("error", `Binary load failed: ${failedIds.join(", ")}`)
        return
      }

      const missingIds = requiredTransformIds.filter((id) => !binaries.has(id))
      if (missingIds.length > 0) {
        setCompiledGraph(null)
        setRuntimeState("hydrating", `Waiting on binaries: ${missingIds.join(", ")}`)
        return
      }

      const descriptor = compileActiveGraph(runtimeGraph, binaries)
      if (!descriptor) {
        setCompiledGraph(null)
        setRuntimeState("error", "Compile failed.")
        return
      }

      setCompiledGraph(descriptor)
      setRuntimeState("idle", null)
    })()
  }, [ensureTransformBinaries, runtimeGraph, setCompiledGraph, setRuntimeState])
}
