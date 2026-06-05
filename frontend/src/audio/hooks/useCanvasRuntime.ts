import { useCallback } from "react"
import type { Edge as RFEdge, Node as RFNode } from "reactflow"
import { useGraphBinaries } from "@/hooks/transforms/queries"
import { runPipeline, type CompileGraphResult } from "@/audio/pipeline/runPipeline"
import { useAudioEffectsStore, type RuntimeStatus } from "@/Stores/AudioEffectsStore"
import { useWorkletStore } from "@/Stores/WorkletStore"

function getRuntimeStatus(result: CompileGraphResult): RuntimeStatus {
  if (result.ok) return "idle"
  if (result.reason === "empty_graph") return "idle"
  if (result.reason === "missing_binaries") return "hydrating"
  return "error"
}

function saveCompileResult(result: CompileGraphResult): void {
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
  const binaries = useGraphBinaries(nodes)

  const compileNow = useCallback((nodesOverride?: RFNode[]): CompileGraphResult => {
    const result = runPipeline({
      graphParams: { graphId, nodes: nodesOverride ?? nodes, edges },
      binaries,
    })
    saveCompileResult(result)
    return result
  }, [graphId, nodes, edges, binaries])

  return { compileNow }
}
