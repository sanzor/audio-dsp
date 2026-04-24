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
    let source: MediaElementAudioSourceNode | null = null
    let workletNode: AudioWorkletNode | null = null

    const setup = async () => {
      const mediaElement = ws.getMediaElement()
      const audioCtx = new AudioContext()
      audioCtxRef.current = audioCtx

      await audioCtx.audioWorklet.addModule(AudioWorkletAdapterUrl)

      source = audioCtx.createMediaElementSource(mediaElement)
      workletNode = new AudioWorkletNode(audioCtx, 'audio-worklet-adapter')

      source.connect(workletNode)
      workletNode.connect(audioCtx.destination)

      graphController.connectWorklet(workletNode.port)
    }

    void setup()

    return () => {
      graphController.disconnectWorklet()
      source?.disconnect()
      workletNode?.disconnect()
      void audioCtxRef.current?.close()
      audioCtxRef.current = null
    }
  }, [graphController])

  const setEffects = useCallback((enabled: boolean) => {
    graphController.setEffects(enabled)
  }, [graphController])

  return { onWaveformBound, setEffects }
}
