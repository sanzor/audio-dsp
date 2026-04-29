import { useWasmBinaryStore } from '@/Stores/WasmBinaryStore'
import type { CompileOutput, CompiledGraph } from '../types/compiled'

// Returns null if any binary is not yet in WasmBinaryStore (still fetching).
export function compute(output: CompileOutput): CompiledGraph | null {
  const { getBinary } = useWasmBinaryStore.getState()

  const wasmBuffers: ArrayBuffer[] = []
  for (const transformId of output.transformIds) {
    const binary = getBinary(transformId)
    if (!binary) return null
    // slice() copies the bytes so the ArrayBuffer can be transferred to the worklet
    // without losing the cached Uint8Array on the main thread.
    wasmBuffers.push(binary.slice().buffer)
  }

  return {
    instructions: output.instructions,
    bufCount: output.bufCount,
    feedbackCount: output.feedbackCount,
    wasmBuffers,
    paramsByInstance: output.paramsByInstance,
  }
}
