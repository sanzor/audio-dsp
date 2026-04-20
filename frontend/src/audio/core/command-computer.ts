import type { CompileOutput, CompiledGraph } from '../types/compiled'
import type { TransformStore } from '../store/TransformStore'

export async function compute(
  output: CompileOutput,
  store: TransformStore,
): Promise<CompiledGraph> {
  const wasmBuffers = await Promise.all(output.transformIds.map(id => store.get(String(id))))

  return {
    instructions: output.instructions,
    bufCount: output.bufCount,
    feedbackCount: output.feedbackCount,
    wasmBuffers,
  }
}
