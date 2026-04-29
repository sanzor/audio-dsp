import type { ActiveGraph } from '@/Stores/ActiveGraphState'
import type { CompileOutput } from '../types/compiled'
import type { Instruction } from '../types/instruction'
import type { FeedbackBuffer, ProcessingPlan } from '../types/processing-plan'
import { createFeedbackBuffers, getFeedbackEdges } from './graph-analysis'
import { buildProcessingPlan } from './graph-planner'

export function compile(graph: ActiveGraph): CompileOutput {
  const feedbackEdges = getFeedbackEdges(graph)
  const feedbackBuffers = createFeedbackBuffers(feedbackEdges)
  const plan = buildProcessingPlan(graph, feedbackBuffers)
  const [instructions, bufCount] = buildInstructions(plan, feedbackBuffers)
  const transformIds = plan.topoOrder.map(id => graph.nodes.get(id)!.transformId)
  const paramsByInstance = plan.topoOrder.map((id) => Object.values(graph.nodes.get(id)!.params ?? {}))

  return { instructions, bufCount, feedbackCount: feedbackBuffers.length, transformIds, paramsByInstance }
}

function buildInstructions(
  plan: ProcessingPlan,
  feedbackBuffers: FeedbackBuffer[],
): [Instruction[], number] {
  let nextBuf = plan.nextBuf
  const instructions: Instruction[] = []

  instructions.push({ kind: 'FetchInput', targetBufIdx: plan.graphInputBuf })

  const fbInjectBufs: number[] = feedbackBuffers.map((_, regIdx) => {
    const buf = nextBuf++
    instructions.push({ kind: 'FetchFeedback', regIdx, targetBufIdx: buf })
    return buf
  })

  for (let instanceIdx = 0; instanceIdx < plan.topoOrder.length; instanceIdx++) {
    const nodeId = plan.topoOrder[instanceIdx]
    const outBuf = plan.nodeOutBuf.get(nodeId)!
    const sources = plan.regInputs.get(nodeId)!
    const feedbacks = plan.fbInputs.get(nodeId)!

    let inputBuf: number
    if (sources.length === 0 && feedbacks.length === 0) {
      inputBuf = plan.graphInputBuf
    } else if (sources.length === 1 && feedbacks.length === 0) {
      inputBuf = plan.nodeOutBuf.get(sources[0])!
    } else if (sources.length === 0 && feedbacks.length === 1) {
      inputBuf = fbInjectBufs[feedbacks[0]]
    } else {
      const mixBuf = nextBuf++
      inputBuf = mixBuf
      for (const src of sources) {
        instructions.push({ kind: 'Sum', sourceBufIdx: plan.nodeOutBuf.get(src)!, targetBufIdx: mixBuf })
      }
      for (const regIdx of feedbacks) {
        instructions.push({ kind: 'Sum', sourceBufIdx: fbInjectBufs[regIdx], targetBufIdx: mixBuf })
      }
    }

    instructions.push({
      kind: 'ExecuteTransform',
      instanceIdx,
      inputBufIdx: inputBuf,
      outputBufIdx: outBuf,
      paramOffset: 0,
    })
  }

  for (let regIdx = 0; regIdx < feedbackBuffers.length; regIdx++) {
    const fb = feedbackBuffers[regIdx]
    const srcBuf = plan.nodeOutBuf.get(fb.fromId)
    if (srcBuf !== undefined) {
      instructions.push({ kind: 'StoreFeedback', sourceBufIdx: srcBuf, regIdx })
    }
  }

  const lastId = plan.topoOrder[plan.topoOrder.length - 1]
  if (lastId !== undefined) {
    instructions.push({ kind: 'WriteOutput', sourceBufIdx: plan.nodeOutBuf.get(lastId)! })
  }

  return [instructions, nextBuf]
}
