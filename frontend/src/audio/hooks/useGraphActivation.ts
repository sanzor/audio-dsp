import { useCallback, useEffect } from "react"
import { useAudioEffectsStore } from "@/Stores/AudioEffectsStore"
import type { CompiledGraph } from "@/audio/types/compiled"

export function useGraphActivation(): { activate: () => void; canActivate: boolean } {
  const graphController = useAudioEffectsStore((s) => s.graphController)
  const compiledGraph = useAudioEffectsStore((s) => s.compiledGraph)
  const workletConnected = useAudioEffectsStore((s) => s.workletConnected)
  const effectsEnabled = useAudioEffectsStore((s) => s.effectsEnabled)
  const setGraphPlaybackState = useAudioEffectsStore((s) => s.setGraphPlaybackState)
  const setRuntimeState = useAudioEffectsStore((s) => s.setRuntimeState)

  const canActivate = compiledGraph != null && workletConnected

  const doActivate = useCallback((compiled: CompiledGraph) => {
    try {
      graphController.loadCompiledGraph(compiled)
      setGraphPlaybackState({ compiled: true, playable: true, reason: null })
      setRuntimeState("ready")
    } catch (error) {
      setGraphPlaybackState({ compiled: true, playable: false, reason: "Activation failed." })
      setRuntimeState("error", error instanceof Error ? error.message : "Activation failed.")
    }
  }, [graphController, setGraphPlaybackState, setRuntimeState])

  // Auto-activate when all conditions are met (graph compiled + worklet ready + effects on).
  useEffect(() => {
    if (compiledGraph && workletConnected && effectsEnabled) {
      doActivate(compiledGraph)
    }
  }, [compiledGraph, workletConnected, effectsEnabled, doActivate])

  const activate = useCallback(() => {
    if (!compiledGraph || !workletConnected) return
    doActivate(compiledGraph)
  }, [compiledGraph, workletConnected, doActivate])

  return { activate, canActivate }
}
