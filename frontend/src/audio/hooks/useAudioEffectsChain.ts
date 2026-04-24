import { useCallback, useRef } from 'react'
import type WaveSurfer from 'wavesurfer.js'
import { useAudioEffectsStore } from '@/Stores/AudioEffectsStore'
import AudioWorkletAdapterUrl from '../worklet/audio-worklet-adapter.ts?url'

export function useAudioEffectsChain() {
  const graphController = useAudioEffectsStore(s => s.graphController)
  const audioCtxRef = useRef<AudioContext | null>(null)

  // Call this when a WaveSurfer instance is bound (alongside playback.bindWaveform).
  // Returns a cleanup function — call it when the waveform unmounts.
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

        await audioCtx.audioWorklet.addModule(AudioWorkletAdapterUrl)
        if (disposed || audioCtx.state === 'closed') return

        source = audioCtx.createMediaElementSource(mediaElement)
        try {
          workletNode = new AudioWorkletNode(audioCtx, 'audio-worklet-adapter')
        } catch (error) {
          if (disposed) return
          if (error instanceof DOMException && error.name === 'InvalidStateError') {
            source?.disconnect()
            source = null
            return
          }
          throw error
        }
        if (disposed || audioCtx.state === 'closed') return

        source.connect(workletNode)
        workletNode.connect(audioCtx.destination)

        graphController.connectWorklet(workletNode.port)
      } catch (error) {
        if (disposed) return
        if (error instanceof DOMException && error.name === 'InvalidStateError') {
          return
        }
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
      if (audioCtxRef.current === audioCtx) {
        audioCtxRef.current = null
      }
    }
  }, [graphController])

  const setEffects = useCallback((enabled: boolean) => {
    graphController.setEffects(enabled)
  }, [graphController])

  return { onWaveformBound, setEffects }
}
