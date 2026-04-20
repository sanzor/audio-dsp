import type { ActiveGraph } from '../types/graph'
import type { CompiledGraph } from '../types/compiled'
import type { TransformStore } from '../store/TransformStore'
import { compile } from './graph-compiler'
import { compute } from './command-computer'

export class AudioPipeline {
  constructor(private readonly store: TransformStore) {}

  async compile(graph: ActiveGraph): Promise<CompiledGraph> {
    const output = compile(graph)
    return compute(output, this.store)
  }
}
