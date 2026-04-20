import type { ActiveGraph } from '../types/graph'
import type { CompiledGraph } from '../types/compiled'
import type { AudioPipeline } from '../core/audio-pipeline'

type WorkletMessage =
  | { type: 'LOAD_GRAPH'; payload: Omit<CompiledGraph, 'wasmBuffers'> & { wasmBuffers: ArrayBuffer[] } }
  | { type: 'SET_EFFECTS'; enabled: boolean }

export class GraphController {
  private compiled: CompiledGraph | null = null
  private workletPort: MessagePort | null = null

  constructor(private readonly pipeline: AudioPipeline) {}

  connectWorklet(port: MessagePort): void {
    this.workletPort = port
  }

  async updateGraph(graph: ActiveGraph): Promise<void> {
    const compiled = await this.pipeline.compile(graph)
    this.compiled = compiled
    this.sendToWorklet(compiled)
  }

  setEffects(enabled: boolean): void {
    this.workletPort?.postMessage({ type: 'SET_EFFECTS', enabled } satisfies WorkletMessage)
  }

  getCompiled(): CompiledGraph | null {
    return this.compiled
  }

  private sendToWorklet(compiled: CompiledGraph): void {
    if (!this.workletPort) return
    const msg: WorkletMessage = { type: 'LOAD_GRAPH', payload: compiled }
    this.workletPort.postMessage(msg, compiled.wasmBuffers)
  }
}
