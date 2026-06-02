import { useCallback, useRef } from 'react'
import type WaveSurfer from 'wavesurfer.js'
import { useWorkletStore } from '@/Stores/WorkletStore'

const GRAPH_WORKLET_URL = new URL('../worklet/graph-worklet.js', import.meta.url).href

export function useWorkletSetup() {
  const graphController = useWorkletStore(s => s.workletController)
  const audioCtxRef = useRef<AudioContext | null>(null)

  const onWaveformBound = useCallback((ws: WaveSurfer): (() => void) => {
    let disposed = false
    let audioCtx: AudioContext | null = null
    let source: MediaElementAudioSourceNode | null = null
    let workletNode: AudioWorkletNode | null = null

    const setup = async () => {
      try {
        const mediaElement = ws.getMediaElement()
        audioCtx = new AudioContext()
        audioCtxRef.current = audioCtx

        await audioCtx.audioWorklet.addModule(GRAPH_WORKLET_URL)
        if (disposed || (audioCtx.state as string) === 'closed') return

        source = audioCtx.createMediaElementSource(mediaElement)
        try {
          workletNode = new AudioWorkletNode(audioCtx, 'graph-worklet')
        } catch (error) {
          if (disposed) return
          if (error instanceof DOMException && error.name === 'InvalidStateError') {
            source?.disconnect()
            source = null
            return
          }
          throw error
        }
        if (disposed || (audioCtx.state as string) === 'closed') return

        source.connect(workletNode)
        workletNode.connect(audioCtx.destination)
        graphController.connectWorklet(workletNode.port)
        graphController.setEffects(useWorkletStore.getState().effectsEnabled)
      } catch (error) {
        if (disposed) return
        if (error instanceof DOMException && error.name === 'InvalidStateError') return
        console.error('Failed to initialize waveform effects chain', error)
      }
    }

    void setup()

    return () => {
      disposed = true
      graphController.disconnectWorklet()
      source?.disconnect()
      workletNode?.disconnect()
      void audioCtx?.close()
      if (audioCtxRef.current === audioCtx) audioCtxRef.current = null
    }
  }, [graphController])

  const setEffects = useCallback((enabled: boolean) => {
    graphController.setEffects(enabled)
  }, [graphController])

  return { onWaveformBound, setEffects }
}
