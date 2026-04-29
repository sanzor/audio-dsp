import type { CompiledGraph } from '../types/compiled'
import type { Instruction } from '../types/instruction'

// Contract expected from each WASM transform module.
export type WasmExports = {
  alloc: (len: number) => number
  process: (inputPtr: number, length: number, paramsPtr: number, paramsLen: number) => void
  memory: WebAssembly.Memory
}

type WasmInstanceState = {
  exports: WasmExports
  inputPtr: number
  paramsPtr: number
  paramsLen: number
}

export type ExecutorState = {
  instances: WasmInstanceState[]
  instructions: Instruction[]
  buffers: Float32Array[]
  feedbackRegistry: Float32Array[]
  blockSize: number
}

export async function createExecutorState(
  compiled: CompiledGraph,
  blockSize: number,
): Promise<ExecutorState> {
  const instances = await Promise.all(
    compiled.wasmBuffers.map(async (bytes, index) => {
      const exports = await WebAssembly.instantiate(bytes).then(
        (r) => r.instance.exports as unknown as WasmExports,
      )
      const params = compiled.paramsByInstance[index] ?? []
      const inputPtr = exports.alloc(blockSize)
      const paramsPtr = exports.alloc(Math.max(params.length, 1))
      new Float32Array(exports.memory.buffer, paramsPtr, params.length).set(params)

      return {
        exports,
        inputPtr,
        paramsPtr,
        paramsLen: params.length,
      }
    }),
  )

  return {
    instances,
    instructions: compiled.instructions,
    buffers: Array.from({ length: compiled.bufCount }, () => new Float32Array(blockSize)),
    feedbackRegistry: Array.from({ length: compiled.feedbackCount }, () => new Float32Array(blockSize)),
    blockSize,
  }
}

// Pure in intent: all state is passed explicitly, no hidden globals.
// Buffers are mutated in-place for performance (allocating per-block is too expensive for audio).
export function processBlock(
  state: ExecutorState,
  input: Float32Array,
  output: Float32Array,
  withEffects: boolean,
): void {
  if (!withEffects || state.instructions.length === 0) {
    output.set(input)
    return
  }

  for (const buf of state.buffers) buf.fill(0)

  for (const instr of state.instructions) {
    switch (instr.kind) {
      case 'FetchInput':
        state.buffers[instr.targetBufIdx].set(input)
        break
      case 'FetchFeedback':
        state.buffers[instr.targetBufIdx].set(state.feedbackRegistry[instr.regIdx])
        break
      case 'Sum':
        for (let i = 0; i < state.blockSize; i++) {
          state.buffers[instr.targetBufIdx][i] += state.buffers[instr.sourceBufIdx][i]
        }
        break
      case 'ExecuteTransform': {
        const inst = state.instances[instr.instanceIdx]
        if (inst) {
          new Float32Array(inst.exports.memory.buffer, inst.inputPtr, state.blockSize)
            .set(state.buffers[instr.inputBufIdx])
          inst.exports.process(inst.inputPtr, state.blockSize, inst.paramsPtr, inst.paramsLen)
          state.buffers[instr.outputBufIdx]
            .set(new Float32Array(inst.exports.memory.buffer, inst.inputPtr, state.blockSize))
        }
        break
      }
      case 'StoreFeedback':
        state.feedbackRegistry[instr.regIdx].set(state.buffers[instr.sourceBufIdx])
        break
      case 'WriteOutput':
        output.set(state.buffers[instr.sourceBufIdx])
        break
    }
  }
}
