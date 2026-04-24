import type { ActiveGraph } from '@/Stores/ActiveGraphState'
import type { CompiledGraph } from '../types/compiled'
import { compile } from './graph-compiler'
import { compute } from './command-computer'

export class AudioPipeline {
  // Returns null if any node binary is not yet in WasmBinaryStore.
  compile(graph: ActiveGraph): CompiledGraph | null {
    const output = compile(graph)
    return compute(output)
  }
}
