import { useCallback, useMemo } from "react"
import type { Edge as RFEdge, Node as RFNode } from "reactflow"
import { buildRuntimeGraph } from "@/audio/pipeline/buildRuntimeGraph"
import { useGraphBinaries } from "@/hooks/transforms/queries"
import { runPipeline, type CompileGraphResult } from "@/audio/pipeline/runPipeline"
import { useAudioEffectsStore, type RuntimeStatus } from "@/Stores/AudioEffectsStore"
import { useWasmBinaryStore } from "@/Stores/WasmBinaryStore"

function getRuntimeStatus(result: CompileGraphResult): RuntimeStatus {
  if (result.ok) return "idle"
  if (result.reason === "empty_graph") return "idle"
  if (result.reason === "missing_binaries") return "hydrating"
  return "error"
}

function applyCompileResult(result: CompileGraphResult): void {
  const { setCompiledGraph, setRuntimeState } = useAudioEffectsStore.getState()
  if (result.ok) {
    setCompiledGraph(result.descriptor)
    setRuntimeState("idle", null)
  } else {
    setCompiledGraph(null)
    setRuntimeState(getRuntimeStatus(result), result.detail)
  }
}

export function useCanvasRuntime(
  graphId: number | undefined,
  nodes: RFNode[],
  edges: RFEdge[],
): { compileNow: () => void } {
  const runtimeGraph = useMemo(
    () => buildRuntimeGraph(graphId, nodes, edges),
    [graphId, nodes, edges],
  )

  useGraphBinaries(runtimeGraph)

  const compileNow = useCallback(() => {
    applyCompileResult(runPipeline(runtimeGraph, useWasmBinaryStore.getState().binaries))
  }, [runtimeGraph])

  return { compileNow }
}
