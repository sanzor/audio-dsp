import { useCallback, useEffect } from "react"
import { useAudioEffectsStore } from "@/Stores/AudioEffectsStore"
import { useWorkletStore } from "@/Stores/WorkletStore"
import type { CompiledGraphResult } from "@/audio/pipeline/compile-graph/compileGraph"

export function useGraphActivation(): { activate: () => void; canActivate: boolean } {
  const workletController = useWorkletStore((s) => s.workletController)
  const compiledGraph = useAudioEffectsStore((s) => s.compiledGraph)
  const workletConnected = useWorkletStore((s) => s.workletConnected)
  const effectsEnabled = useWorkletStore((s) => s.effectsEnabled)
  const setGraphPlaybackState = useWorkletStore((s) => s.setGraphPlaybackState)
  const setWorkletState = useWorkletStore((s) => s.setWorkletState)

  const canActivate = compiledGraph != null && workletConnected

  const doActivate = useCallback((compiled: CompiledGraphResult) => {
    try {
      workletController.loadCompiledGraphToWorklet(compiled)
      setGraphPlaybackState({ compiled: true, playable: false, reason: "Waiting for the worklet to finish loading the graph." })
      setWorkletState("sending", "Loading graph in audio worklet...")
    } catch (error) {
      setGraphPlaybackState({ compiled: true, playable: false, reason: "Activation failed." })
      setWorkletState("error", error instanceof Error ? error.message : "Activation failed.")
    }
  }, [workletController, setGraphPlaybackState, setWorkletState])

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
