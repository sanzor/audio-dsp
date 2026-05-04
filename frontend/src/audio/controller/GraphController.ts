import type { ActiveGraph } from '@/Stores/ActiveGraphState'
import type { CompiledGraph } from '../types/compiled'
import type { AudioPipeline } from '../core/audio-pipeline'

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

  compileGraph(graph: ActiveGraph): CompiledGraph | null {
    return this.pipeline.compile(graph)
  }

  loadCompiledGraph(compiled: CompiledGraph): void {
    this.sendToWorklet({ type: 'LOAD_GRAPH', payload: compiled })
  }

  updateGraph(graph: ActiveGraph): boolean {
    const compiled = this.compileGraph(graph)
    if (!compiled) {
      return false
    }
    this.loadCompiledGraph(compiled)
    return true
  }

  setEffects(enabled: boolean): void {
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
