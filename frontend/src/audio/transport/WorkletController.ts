import { WorkletMessageSender } from './WorkletMessageSender'
import type { CompiledGraphResult } from '@/audio/pipeline/compile-graph/compileGraph'
import { useWorkletStore } from '@/Stores/WorkletStore'
import { useAudioEffectsStore } from '@/Stores/AudioEffectsStore'

export class WorkletController {
  private sender: WorkletMessageSender | null = null

  connectWorklet(port: MessagePort): void {
    const sender = new WorkletMessageSender(port)
    this.sender = sender

    sender.onGraphReady(() => {
      const { setGraphPlaybackState, setWorkletState } = useWorkletStore.getState()
      setGraphPlaybackState({ compiled: true, playable: true, reason: null })
      setWorkletState('ready')
    })

    sender.onModuleError((transformId, error) => {
      const { setGraphPlaybackState, setWorkletState } = useWorkletStore.getState()
      setGraphPlaybackState({ compiled: true, playable: false, reason: `Transform ${transformId} failed to load in the worklet.` })
      setWorkletState('error', error)
    })

    useWorkletStore.getState().setWorkletConnected(true)
  }

  disconnectWorklet(): void {
    this.sender?.dispose()
    this.sender = null

    const compiled = useAudioEffectsStore.getState().compiledGraph != null
    const { setWorkletConnected, setGraphPlaybackState } = useWorkletStore.getState()
    setWorkletConnected(false)
    setGraphPlaybackState({ compiled, playable: false, reason: 'Worklet is disconnected.' })
  }

  loadCompiledGraphToWorklet(descriptor: CompiledGraphResult): void {
    if (!this.sender) return
    const { setGraphPlaybackState, setWorkletState } = useWorkletStore.getState()
    setWorkletState('sending', 'Loading graph in audio worklet...')
    setGraphPlaybackState({ compiled: true, playable: false, reason: 'Waiting for the worklet to finish loading the graph.' })
    this.sender.sendGraph(descriptor.compiledGraph, descriptor.resolved_binaries)
  }

  setEffects(enabled: boolean): void {
    this.sender?.sendBypass(!enabled)
  }
}
