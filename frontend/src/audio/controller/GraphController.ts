import type { CompiledGraph } from '../types/compiled'

type WorkletMessage =
  | { type: 'LOAD_GRAPH'; payload: CompiledGraph }
  | { type: 'SET_EFFECTS'; enabled: boolean }

export class GraphController {
  private workletPort: MessagePort | null = null

  connectWorklet(port: MessagePort): void {
    this.workletPort = port
  }

  disconnectWorklet(): void {
    this.workletPort = null
  }

  loadCompiledGraph(compiled: CompiledGraph): void {
    this.sendToWorklet({ type: 'LOAD_GRAPH', payload: compiled })
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
