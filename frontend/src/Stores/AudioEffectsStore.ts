import { create } from 'zustand'
import type { CompiledGraphResult } from '@/audio/pipeline/compile-graph/compileGraph'

export type RuntimeStatus = 'idle' | 'hydrating' | 'error'
export type CompileLogLevel = 'info' | 'success' | 'warning' | 'error'
export type CompileRunStatus = 'running' | 'success' | 'error'
export type CompileStepStatus = 'pending' | 'running' | 'success' | 'warning' | 'error'

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

const DEFAULT_COMPILE_STEPS = (): CompileStep[] => ([
  { id: 'shape', label: 'Inspect graph shape', status: 'pending', detail: null },
  { id: 'binaries', label: 'Resolve transform binaries', status: 'pending', detail: null },
  { id: 'compile', label: 'Build compiled graph', status: 'pending', detail: null },
  { id: 'activate', label: 'Ready to activate', status: 'pending', detail: null },
])

let nextCompileRunId = 1
let nextCompileLogId = 1

interface AudioEffectsState {
  compiledGraph: CompiledGraphResult | null
  compileModalOpen: boolean
  compileRun: CompileRun | null
  runtimeStatus: RuntimeStatus
  runtimeMessage: string | null
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
  setRuntimeState: (status: RuntimeStatus, message?: string | null) => void
}

export const useAudioEffectsStore = create<AudioEffectsState>()((set) => ({
  compiledGraph: null,
  compileModalOpen: false,
  compileRun: null,
  runtimeStatus: 'idle',
  runtimeMessage: null,
  setCompiledGraph: (compiledGraph) => set({ compiledGraph }),
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
          steps: state.compileRun.steps.map((step) =>
            step.id !== stepId ? step : { ...step, ...patch }
          ),
        },
  })),
  appendCompileLog: (runId, level, message) => set((state) => ({
    compileRun: state.compileRun?.id !== runId
      ? state.compileRun
      : {
          ...state.compileRun,
          logs: [
            ...state.compileRun.logs,
            { id: nextCompileLogId++, at: Date.now(), level, message },
          ].slice(-200),
        },
  })),
  finishCompileRun: (runId, status) => set((state) => ({
    compileRun: state.compileRun?.id !== runId
      ? state.compileRun
      : { ...state.compileRun, status, finishedAt: Date.now() },
  })),
  setRuntimeState: (runtimeStatus, runtimeMessage = null) => set({ runtimeStatus, runtimeMessage }),
}))
