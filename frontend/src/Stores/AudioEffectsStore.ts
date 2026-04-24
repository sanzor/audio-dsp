import { create } from 'zustand'
import { AudioPipeline } from '@/audio/core/audio-pipeline'
import { GraphController } from '@/audio/controller/GraphController'

const pipeline = new AudioPipeline()
const graphController = new GraphController(pipeline)

interface AudioEffectsState {
  hasCompiledGraph: boolean
  withEffects: boolean
  graphController: GraphController
  setHasCompiledGraph: (v: boolean) => void
  setWithEffects: (v: boolean) => void
}

export const useAudioEffectsStore = create<AudioEffectsState>()((set) => ({
  hasCompiledGraph: false,
  withEffects: true,
  graphController,
  setHasCompiledGraph: (v) => set({ hasCompiledGraph: v }),
  setWithEffects: (v) => set({ withEffects: v }),
}))
