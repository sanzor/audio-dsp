import type { ActiveGraph } from '@/Stores/ActiveGraphState'
import type { CompiledGraph } from '../types/compiled'
import type { AudioPipeline } from '../core/audio-pipeline'
import { useAudioEffectsStore } from '@/Stores/AudioEffectsStore'

type WorkletMessage =
  | { type: 'LOAD_GRAPH'; payload: CompiledGraph }
  | { type: 'SET_EFFECTS'; enabled: boolean }

export class GraphController {
  private workletPort: MessagePort | null = null

  constructor(private readonly pipeline: AudioPipeline) {}

  connectWorklet(port: MessagePort): void {
    this.workletPort = port
  }

  disconnectWorklet(): void {
    this.workletPort = null
  }

  updateGraph(graph: ActiveGraph): void {
    const compiled = this.pipeline.compile(graph)
    if (!compiled) return
    useAudioEffectsStore.getState().setHasCompiledGraph(true)
    this.sendToWorklet({ type: 'LOAD_GRAPH', payload: compiled })
  }

  setEffects(enabled: boolean): void {
    useAudioEffectsStore.getState().setWithEffects(enabled)
    this.sendToWorklet({ type: 'SET_EFFECTS', enabled })
  }

  private sendToWorklet(msg: WorkletMessage): void {
    if (!this.workletPort) return
    if (msg.type === 'LOAD_GRAPH') {
      this.workletPort.postMessage(msg, msg.payload.wasmBuffers)
    } else {
      this.workletPort.postMessage(msg)
    }
  }
}
