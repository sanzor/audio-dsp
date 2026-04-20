import type { CompiledGraph } from '../types/compiled'
import { createExecutorState, processBlock } from '../core/audio-executor'
import type { ExecutorState } from '../core/audio-executor'

type InboundMessage =
  | { type: 'LOAD_GRAPH'; payload: CompiledGraph }
  | { type: 'SET_EFFECTS'; enabled: boolean }

const BLOCK_SIZE = 128

class AudioWorkletAdapter extends AudioWorkletProcessor {
  private state: ExecutorState | null = null
  private withEffects = true

  constructor() {
    super()
    this.port.onmessage = (e: MessageEvent<InboundMessage>) => {
      void this.handleMessage(e.data)
    }
  }

  private async handleMessage(msg: InboundMessage): Promise<void> {
    if (msg.type === 'LOAD_GRAPH') {
      this.state = await createExecutorState(msg.payload, BLOCK_SIZE)
    } else if (msg.type === 'SET_EFFECTS') {
      this.withEffects = msg.enabled
    }
  }

  process(inputs: Float32Array[][], outputs: Float32Array[][]): boolean {
    const input = inputs[0]?.[0] ?? new Float32Array(BLOCK_SIZE)
    const output = outputs[0]?.[0]
    if (!output) return true

    if (!this.state) {
      output.set(input)
      return true
    }

    processBlock(this.state, input, output, this.withEffects)
    return true
  }
}

registerProcessor('audio-worklet-adapter', AudioWorkletAdapter)
