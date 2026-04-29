import type { Instruction } from './instruction'

export type CompileOutput = {
  instructions: Instruction[]
  bufCount: number
  feedbackCount: number
  transformIds: number[]
  paramsByInstance: number[][]
}

export type CompiledGraph = {
  instructions: Instruction[]
  bufCount: number
  feedbackCount: number
  wasmBuffers: ArrayBuffer[]
  paramsByInstance: number[][]
}
