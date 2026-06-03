import { useCallback, useMemo } from "react"
import type { Edge as RFEdge, Node as RFNode } from "reactflow"
import { buildRuntimeGraph } from "@/audio/pipeline/buildRuntimeGraph"
import { useGraphBinaries } from "@/hooks/transforms/queries"
import { runPipeline, type CompileGraphResult } from "@/audio/pipeline/runPipeline"
import { useAudioEffectsStore, type RuntimeStatus } from "@/Stores/AudioEffectsStore"
import { useWorkletStore } from "@/Stores/WorkletStore"
import { useWasmBinaryStore } from "@/Stores/WasmBinaryStore"

function getRuntimeStatus(result: CompileGraphResult): RuntimeStatus {
  if (result.ok) return "idle"
  if (result.reason === "empty_graph") return "idle"
  if (result.reason === "missing_binaries") return "hydrating"
  return "error"
}

function applyCompileResult(result: CompileGraphResult): void {
  const { setCompiledGraph, setRuntimeState } = useAudioEffectsStore.getState()
  const { setGraphPlaybackState } = useWorkletStore.getState()
  if (result.ok) {
    setCompiledGraph(result.descriptor)
    setGraphPlaybackState({ compiled: true, playable: false, reason: "Compiled graph is ready to activate." })
    setRuntimeState("idle", null)
  } else {
    setCompiledGraph(null)
    setGraphPlaybackState({ compiled: false, playable: false, reason: null })
    setRuntimeState(getRuntimeStatus(result), result.detail)
  }
}

export function useCanvasRuntime(
  graphId: number | undefined,
  nodes: RFNode[],
  edges: RFEdge[],
): { compileNow: (nodesOverride?: RFNode[]) => CompileGraphResult } {
  const runtimeGraph = useMemo(
    () => buildRuntimeGraph(graphId, nodes, edges),
    [graphId, nodes, edges],
  )

  useGraphBinaries(runtimeGraph)

  const compileNow = useCallback((nodesOverride?: RFNode[]): CompileGraphResult => {
    const graph = nodesOverride
      ? buildRuntimeGraph(graphId, nodesOverride, edges)
      : runtimeGraph
    const result = runPipeline(graph, useWasmBinaryStore.getState().binaries)
    applyCompileResult(result)
    return result
  }, [graphId, edges, runtimeGraph])

  return { compileNow }
}
