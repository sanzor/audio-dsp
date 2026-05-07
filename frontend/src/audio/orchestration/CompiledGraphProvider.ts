import type { CompiledGraph } from "@/audio/types/compiled"
import { useAudioEffectsStore } from "@/Stores/AudioEffectsStore"

export interface CompiledGraphProvider {
  setCompiledGraph(graph: CompiledGraph | null): void
}

export class AudioEffectsCompiledGraphProvider implements CompiledGraphProvider {
  setCompiledGraph(graph: CompiledGraph | null): void {
    useAudioEffectsStore.getState().setCompiledGraph(graph)
  }
}
