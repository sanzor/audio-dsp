import { create } from 'zustand'
import { WorkletController } from '@/audio/transport/WorkletController'

const workletController = new WorkletController()

export type WorkletStatus = 'idle' | 'sending' | 'ready' | 'error'

export interface GraphPlaybackState {
  compiled: boolean
  playable: boolean
  reason: string | null
}

const DEFAULT_PLAYBACK_STATE: GraphPlaybackState = {
  compiled: false,
  playable: false,
  reason: null,
}

interface WorkletState {
  workletController: WorkletController
  workletConnected: boolean
  workletStatus: WorkletStatus
  workletMessage: string | null
  graphPlaybackState: GraphPlaybackState
  effectsEnabled: boolean
  setWorkletConnected: (v: boolean) => void
  setWorkletState: (status: WorkletStatus, message?: string | null) => void
  setGraphPlaybackState: (state: GraphPlaybackState) => void
  setEffectsEnabled: (v: boolean) => void
  toggleEffectsEnabled: () => void
  isGraphPlayable: () => boolean
}

export const useWorkletStore = create<WorkletState>()((set, get) => ({
  workletController,
  workletConnected: false,
  workletStatus: 'idle',
  workletMessage: null,
  graphPlaybackState: DEFAULT_PLAYBACK_STATE,
  effectsEnabled: false,
  setWorkletConnected: (workletConnected) => set({ workletConnected }),
  setWorkletState: (workletStatus, workletMessage = null) => set({ workletStatus, workletMessage }),
  setGraphPlaybackState: (graphPlaybackState) => set({ graphPlaybackState }),
  setEffectsEnabled: (effectsEnabled) => set({ effectsEnabled }),
  toggleEffectsEnabled: () => set((s) => ({ effectsEnabled: !s.effectsEnabled })),
  isGraphPlayable: () => {
    const { graphPlaybackState, effectsEnabled, workletConnected } = get()
    return graphPlaybackState.compiled && graphPlaybackState.playable && effectsEnabled && workletConnected
  },
}))
