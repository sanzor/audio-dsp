import { useCallback, useEffect } from "react"
import { useAudioEffectsStore } from "@/Stores/AudioEffectsStore"
import { useWorkletStore } from "@/Stores/WorkletStore"

export function useWorklet(): { uploadToWorklet: () => void; canUploadToWorklet: boolean } {
  const workletController = useWorkletStore((s) => s.workletController)
  const compiledGraph = useAudioEffectsStore((s) => s.compiledGraph)
  const workletConnected = useWorkletStore((s) => s.workletConnected)
  const effectsEnabled = useWorkletStore((s) => s.effectsEnabled)

  const canUploadToWorklet = compiledGraph != null && workletConnected

  useEffect(() => {
    if (compiledGraph && workletConnected && effectsEnabled) {
      workletController.loadCompiledGraphToWorklet(compiledGraph)
    }
  }, [compiledGraph, workletConnected, effectsEnabled, workletController])

  const uploadToWorklet = useCallback(() => {
    if (!compiledGraph || !workletConnected) return
    workletController.loadCompiledGraphToWorklet(compiledGraph)
  }, [compiledGraph, workletConnected, workletController])

  return { uploadToWorklet,  canUploadToWorklet }
}
