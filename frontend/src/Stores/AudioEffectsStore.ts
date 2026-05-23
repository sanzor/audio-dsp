import { create } from 'zustand'
import { WorkletController } from '@/audio/transport/WorkletController'
import type { CompiledGraphResult } from '@/audio/pipeline/compileRuntimeGraph'

const graphController = new WorkletController()

export type RuntimeStatus = 'idle' | 'hydrating' | 'ready' | 'error'
export type CompileLogLevel = 'info' | 'success' | 'warning' | 'error'
export type CompileRunStatus = 'running' | 'success' | 'error'
export type CompileStepStatus = 'pending' | 'running' | 'success' | 'warning' | 'error'

export interface GraphPlaybackState {
  compiled: boolean
  playable: boolean
  reason: string | null
}

export interface CompileStep {
  id: 'shape' | 'binaries' | 'compile' | 'activate'
  label: string
  status: CompileStepStatus
  detail: string | null
}

export interface CompileLogEntry {
  id: number
  at: number
  level: CompileLogLevel
  message: string
}

export interface CompileRun {
  id: number
  trigger: 'manual' | 'auto'
  startedAt: number
  finishedAt: number | null
  status: CompileRunStatus
  steps: CompileStep[]
  logs: CompileLogEntry[]
}

const DEFAULT_GRAPH_PLAYBACK_STATE: GraphPlaybackState = {
  compiled: false,
  playable: false,
  reason: null,
}

const DEFAULT_COMPILE_STEPS = (): CompileStep[] => ([
  { id: 'shape', label: 'Inspect graph shape', status: 'pending', detail: null },
  { id: 'binaries', label: 'Resolve transform binaries', status: 'pending', detail: null },
  { id: 'compile', label: 'Build compiled graph', status: 'pending', detail: null },
  { id: 'activate', label: 'Ready to activate', status: 'pending', detail: null },
])

let nextCompileRunId = 1
let nextCompileLogId = 1

interface AudioEffectsState {
  graphPlaybackState: GraphPlaybackState
  compiledGraph: CompiledGraphResult | null
  compileModalOpen: boolean
  compileRun: CompileRun | null
  effectsEnabled: boolean
  workletConnected: boolean
  runtimeStatus: RuntimeStatus
  runtimeMessage: string | null
  workletController: WorkletController
  setGraphPlaybackState: (graphPlaybackState: GraphPlaybackState) => void
  setCompiledGraph: (compiled: CompiledGraphResult | null) => void
  setCompileModalOpen: (open: boolean) => void
  clearCompileRun: () => void
  beginCompileRun: (trigger: 'manual' | 'auto', openModal?: boolean) => number
  updateCompileStep: (
    runId: number,
    stepId: CompileStep['id'],
    patch: Partial<Pick<CompileStep, 'status' | 'detail'>>,
  ) => void
  appendCompileLog: (runId: number, level: CompileLogLevel, message: string) => void
  finishCompileRun: (runId: number, status: CompileRunStatus) => void
  setEffectsEnabled: (v: boolean) => void
  setWorkletConnected: (v: boolean) => void
  setRuntimeState: (status: RuntimeStatus, message?: string | null) => void
  isGraphPlayable: () => boolean
  toggleEffectsEnabled: () => void
}

export const useAudioEffectsStore = create<AudioEffectsState>()((set, get) => ({
  graphPlaybackState: DEFAULT_GRAPH_PLAYBACK_STATE,
  compiledGraph: null,
  compileModalOpen: false,
  compileRun: null,
  effectsEnabled: false,
  workletConnected: false,
  runtimeStatus: 'idle',
  runtimeMessage: null,
  workletController: graphController,
  setGraphPlaybackState: (graphPlaybackState) => set({ graphPlaybackState }),
  setCompiledGraph: (compiledGraph) => set({
    compiledGraph,
    graphPlaybackState: compiledGraph == null
      ? { compiled: false, playable: false, reason: null }
      : { compiled: true, playable: false, reason: "Compiled graph is ready to activate." },
  }),
  setCompileModalOpen: (compileModalOpen) => set({ compileModalOpen }),
  clearCompileRun: () => set({ compileRun: null }),
  beginCompileRun: (trigger, openModal = false) => {
    const runId = nextCompileRunId++
    const run: CompileRun = {
      id: runId,
      trigger,
      startedAt: Date.now(),
      finishedAt: null,
      status: 'running',
      steps: DEFAULT_COMPILE_STEPS(),
      logs: [],
    }

    set((state) => ({
      compileModalOpen: openModal || state.compileModalOpen,
      compileRun: run,
    }))

    return runId
  },
  updateCompileStep: (runId, stepId, patch) => set((state) => ({
    compileRun: state.compileRun?.id !== runId
      ? state.compileRun
      : {
          ...state.compileRun,
          steps: state.compileRun.steps.map((step) => (
            step.id !== stepId
              ? step
              : {
                  ...step,
                  ...patch,
                }
          )),
        },
  })),
  appendCompileLog: (runId, level, message) => set((state) => ({
    compileRun: state.compileRun?.id !== runId
      ? state.compileRun
      : {
          ...state.compileRun,
          logs: [
            ...state.compileRun.logs,
            {
              id: nextCompileLogId++,
              at: Date.now(),
              level,
              message,
            },
          ].slice(-200),
        },
  })),
  finishCompileRun: (runId, status) => set((state) => ({
    compileRun: state.compileRun?.id !== runId
      ? state.compileRun
      : {
          ...state.compileRun,
          status,
          finishedAt: Date.now(),
        },
  })),
  setEffectsEnabled: (v) => set({ effectsEnabled: v }),
  setWorkletConnected: (v) => set({ workletConnected: v }),
  setRuntimeState: (runtimeStatus, runtimeMessage = null) => set({ runtimeStatus, runtimeMessage }),
  isGraphPlayable: () => {
    const { graphPlaybackState, effectsEnabled, workletConnected } = get()
    return graphPlaybackState.compiled && graphPlaybackState.playable && effectsEnabled && workletConnected
  },
  toggleEffectsEnabled: () => set((state) => ({ effectsEnabled: !state.effectsEnabled })),
}))
