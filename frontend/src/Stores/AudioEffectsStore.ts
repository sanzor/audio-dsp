import { create } from 'zustand'
import { AudioPipeline } from '@/audio/core/audio-pipeline'
import { GraphController } from '@/audio/controller/GraphController'

const pipeline = new AudioPipeline()
const graphController = new GraphController(pipeline)

export type RuntimeStatus = 'idle' | 'hydrating' | 'ready' | 'error'

interface AudioEffectsState {
  hasCompiledGraph: boolean
  effectsEnabled: boolean
  workletConnected: boolean
  runtimeStatus: RuntimeStatus
  runtimeMessage: string | null
  graphController: GraphController
  setHasCompiledGraph: (v: boolean) => void
  setEffectsEnabled: (v: boolean) => void
  setWorkletConnected: (v: boolean) => void
  setRuntimeState: (status: RuntimeStatus, message?: string | null) => void
  toggleEffectsEnabled: () => void
}

export const useAudioEffectsStore = create<AudioEffectsState>()((set) => ({
  hasCompiledGraph: false,
  effectsEnabled: false,
  workletConnected: false,
  runtimeStatus: 'idle',
  runtimeMessage: null,
  graphController,
  setHasCompiledGraph: (v) => set({ hasCompiledGraph: v }),
  setEffectsEnabled: (v) => set({ effectsEnabled: v }),
  setWorkletConnected: (v) => set({ workletConnected: v }),
  setRuntimeState: (runtimeStatus, runtimeMessage = null) => set({ runtimeStatus, runtimeMessage }),
  toggleEffectsEnabled: () => set((state) => ({ effectsEnabled: !state.effectsEnabled })),
}))
