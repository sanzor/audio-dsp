import { useCallback } from "react"
import { useWasmBinaryStore } from "@/Stores/WasmBinaryStore"
import type { ActiveGraph } from "@/Stores/ActiveGraphState"
import { PipelineOrchestrator } from "@/audio/orchestration/PipelineProvider"
import { AudioEffectsCompiledGraphProvider } from "@/audio/orchestration/CompiledGraphProvider"
import { AudioEffectsRuntimeStateProvider } from "@/audio/orchestration/RuntimeStateProvider"

const orchestrator = new PipelineOrchestrator(
  new AudioEffectsCompiledGraphProvider(),
  new AudioEffectsRuntimeStateProvider(),
)

export function useCompile(runtimeGraph: ActiveGraph | null): () => void {
  const binaries = useWasmBinaryStore((s) => s.binaries)
  const binaryStatus = useWasmBinaryStore((s) => s.status)

  return useCallback(() => {
    orchestrator.apply({ graph: runtimeGraph, binaries, binaryStatus })
  }, [runtimeGraph, binaries, binaryStatus])
}
