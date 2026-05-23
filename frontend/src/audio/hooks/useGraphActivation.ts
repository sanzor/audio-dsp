import { useCallback, useEffect } from "react"
import { useAudioEffectsStore } from "@/Stores/AudioEffectsStore"
import type { CompiledGraphResult } from "@/audio/pipeline/compileRuntimeGraph"

export function useGraphActivation(): { activate: () => void; canActivate: boolean } {
  const workletController = useAudioEffectsStore((s) => s.workletController)
  const compiledGraph = useAudioEffectsStore((s) => s.compiledGraph)
  const workletConnected = useAudioEffectsStore((s) => s.workletConnected)
  const effectsEnabled = useAudioEffectsStore((s) => s.effectsEnabled)
  const setGraphPlaybackState = useAudioEffectsStore((s) => s.setGraphPlaybackState)
  const setRuntimeState = useAudioEffectsStore((s) => s.setRuntimeState)

  const canActivate = compiledGraph != null && workletConnected

  const doActivate = useCallback((compiled: CompiledGraphResult) => {
    try {
      workletController.loadCompiledGraphToWorklet(compiled)
      setGraphPlaybackState({
        compiled: true,
        playable: false,
        reason: "Waiting for the worklet to finish loading the graph.",
      })
      setRuntimeState("hydrating", "Loading graph in audio worklet...")
    } catch (error) {
      setGraphPlaybackState({ compiled: true, playable: false, reason: "Activation failed." })
      setRuntimeState("error", error instanceof Error ? error.message : "Activation failed.")
    }
  }, [workletController, setGraphPlaybackState, setRuntimeState])

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
