import { create } from 'zustand'
import { AudioPipeline } from '@/audio/core/audio-pipeline'
import { GraphController } from '@/audio/controller/GraphController'

const pipeline = new AudioPipeline()
const graphController = new GraphController(pipeline)

export type RuntimeStatus = 'idle' | 'hydrating' | 'ready' | 'error'
export interface GraphPlaybackState {
  compiled: boolean
  playable: boolean
  reason: string | null
}

const DEFAULT_GRAPH_PLAYBACK_STATE: GraphPlaybackState = {
  compiled: false,
  playable: false,
  reason: null,
}

interface AudioEffectsState {
  graphPlaybackState: GraphPlaybackState
  effectsEnabled: boolean
  workletConnected: boolean
  runtimeStatus: RuntimeStatus
  runtimeMessage: string | null
  graphController: GraphController
  setGraphPlaybackState: (graphPlaybackState: GraphPlaybackState) => void
  setEffectsEnabled: (v: boolean) => void
  setWorkletConnected: (v: boolean) => void
  setRuntimeState: (status: RuntimeStatus, message?: string | null) => void
  isGraphPlayable: () => boolean
  toggleEffectsEnabled: () => void
}

export const useAudioEffectsStore = create<AudioEffectsState>()((set, get) => ({
  graphPlaybackState: DEFAULT_GRAPH_PLAYBACK_STATE,
  effectsEnabled: false,
  workletConnected: false,
  runtimeStatus: 'idle',
  runtimeMessage: null,
  graphController,
  setGraphPlaybackState: (graphPlaybackState) => set({ graphPlaybackState }),
  setEffectsEnabled: (v) => set({ effectsEnabled: v }),
  setWorkletConnected: (v) => set({ workletConnected: v }),
  setRuntimeState: (runtimeStatus, runtimeMessage = null) => set({ runtimeStatus, runtimeMessage }),
  isGraphPlayable: () => {
    const { graphPlaybackState, effectsEnabled, workletConnected } = get()
    return graphPlaybackState.compiled && graphPlaybackState.playable && effectsEnabled && workletConnected
  },
  toggleEffectsEnabled: () => set((state) => ({ effectsEnabled: !state.effectsEnabled })),
}))
