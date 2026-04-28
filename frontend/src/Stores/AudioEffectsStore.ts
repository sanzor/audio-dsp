import { create } from 'zustand'
import { AudioPipeline } from '@/audio/core/audio-pipeline'
import { GraphController } from '@/audio/controller/GraphController'

const pipeline = new AudioPipeline()
const graphController = new GraphController(pipeline)

interface AudioEffectsState {
  hasCompiledGraph: boolean
  effectsEnabled: boolean
  graphController: GraphController
  setHasCompiledGraph: (v: boolean) => void
  setEffectsEnabled: (v: boolean) => void
  toggleEffectsEnabled: () => void
}

export const useAudioEffectsStore = create<AudioEffectsState>()((set) => ({
  hasCompiledGraph: false,
  effectsEnabled: false,
  graphController,
  setHasCompiledGraph: (v) => set({ hasCompiledGraph: v }),
  setEffectsEnabled: (v) => set({ effectsEnabled: v }),
  toggleEffectsEnabled: () => set((state) => ({ effectsEnabled: !state.effectsEnabled })),
}))
