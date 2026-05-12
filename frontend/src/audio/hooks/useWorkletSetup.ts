import { useCallback, useRef } from 'react'
import type WaveSurfer from 'wavesurfer.js'
import { useAudioEffectsStore } from '@/Stores/AudioEffectsStore'

const GRAPH_WORKLET_URL = new URL('../worklet/graph-worklet.js', import.meta.url).href

export function useWorkletSetup() {
  const graphController = useAudioEffectsStore(s => s.graphController)
  const setWorkletConnected = useAudioEffectsStore(s => s.setWorkletConnected)
  const setGraphPlaybackState = useAudioEffectsStore(s => s.setGraphPlaybackState)
  const setRuntimeState = useAudioEffectsStore(s => s.setRuntimeState)
  const audioCtxRef = useRef<AudioContext | null>(null)

  // Call this when a WaveSurfer instance is bound (alongside playback.bindWaveform).
  // Returns a cleanup function — call it when the waveform unmounts.
  const onWaveformBound = useCallback((ws: WaveSurfer): (() => void) => {
    let disposed = false
    let audioCtx: AudioContext | null = null
    let source: MediaElementAudioSourceNode | null = null
    let workletNode: AudioWorkletNode | null = null
    let cleanupGraphReady = () => {}
    let cleanupModuleError = () => {}

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

        const sender = graphController.connectWorklet(workletNode.port)
        cleanupGraphReady = sender.onGraphReady(() => {
          setGraphPlaybackState({ compiled: true, playable: true, reason: null })
          setRuntimeState('ready')
        })
        cleanupModuleError = sender.onModuleError((transformId, error) => {
          setGraphPlaybackState({
            compiled: true,
            playable: false,
            reason: `Transform ${transformId} failed to load in the worklet.`,
          })
          setRuntimeState('error', error)
        })
        setWorkletConnected(true)
        graphController.setEffects(useAudioEffectsStore.getState().effectsEnabled)
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
      cleanupGraphReady()
      cleanupModuleError()
      graphController.disconnectWorklet()
      setWorkletConnected(false)
      setGraphPlaybackState({
        compiled: useAudioEffectsStore.getState().compiledGraph != null,
        playable: false,
        reason: 'Worklet is disconnected.',
      })
      source?.disconnect()
      workletNode?.disconnect()
      void audioCtx?.close()
      if (audioCtxRef.current === audioCtx) {
        audioCtxRef.current = null
      }
    }
  }, [graphController, setGraphPlaybackState, setRuntimeState, setWorkletConnected])

  const setEffects = useCallback((enabled: boolean) => {
    graphController.setEffects(enabled)
  }, [graphController])

  return { onWaveformBound, setEffects }
}
