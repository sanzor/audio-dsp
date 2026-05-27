import { useCallback, useRef } from "react"
import { useAudioEffectsStore } from "@/Stores/AudioEffectsStore"
import { useWasmBinaryStore } from "@/Stores/WasmBinaryStore"
import type { ActiveGraph } from "@/Stores/ActiveGraphState"
import { ensureTransformBinariesExist } from "@/hooks/transforms/useEnsureTransformBinaries"
import {
  compileGraph,
  type CompileGraphResult,
} from "@/audio/pipeline/compileGraph"
import {
  prepareRuntimeGraphCompile,
  validateRuntimeGraph,
} from "@/audio/pipeline/validateRuntimeGraph"

function getRuntimeStatus(result: CompileGraphResult): "idle" | "hydrating" | "error" {
  if (result.ok) return "idle"
  if (result.reason === "empty_graph") return "idle"
  if (result.reason === "missing_binaries") return "hydrating"
  return "error"
}

export function useCompile(): (runtimeGraph: ActiveGraph | null) => Promise<CompileGraphResult> {
  const ensureTransformBinariesExistFunc = ensureTransformBinariesExist()
  const setCompiledGraph = useAudioEffectsStore((s) => s.setCompiledGraph)
  const setRuntimeState = useAudioEffectsStore((s) => s.setRuntimeState)
  const compileSequenceRef = useRef(0)

  return useCallback(async (runtimeGraph: ActiveGraph | null) => {
    const sequence = ++compileSequenceRef.current
    const validated = validateRuntimeGraph(runtimeGraph)
    if (!validated.ok) {
      if (sequence === compileSequenceRef.current) {
        if (validated.reason === "empty_graph") {
          setCompiledGraph(null)
          setRuntimeState("idle", validated.detail)
        } else {
          setCompiledGraph(null)
          setRuntimeState(getRuntimeStatus(validated), validated.detail)
        }
      }

      return validated
    }

    const getDeps = () => {
      const state = useWasmBinaryStore.getState()
      return {
        binaries: state.binaries,
        statuses: state.status,
      }
    }

    const preflight = prepareRuntimeGraphCompile(validated.value, getDeps())
    if (!preflight.ok && preflight.reason === "missing_binaries") {
      if (sequence === compileSequenceRef.current) {
        setCompiledGraph(null)
        setRuntimeState("hydrating", preflight.detail)
      }

      try {
        await ensureTransformBinariesExistFunc(validated.value.transformIds)
      } catch (error) {
        const failedResult: CompileGraphResult = {
          ok: false,
          reason: "binary_load_failed",
          detail: error instanceof Error ? error.message : "Failed to fetch transform binaries.",
          transformIds: validated.value.transformIds,
        }

        if (sequence === compileSequenceRef.current) {
          setCompiledGraph(null)
          setRuntimeState("error", failedResult.detail)
        }

        return failedResult
      }
    }

    const prepared = prepareRuntimeGraphCompile(validated.value, getDeps())
    if (!prepared.ok) {
      if (sequence !== compileSequenceRef.current) {
        return prepared
      }

      setCompiledGraph(null)
      setRuntimeState(getRuntimeStatus(prepared), prepared.detail)
      return prepared
    }

    const result = compileGraph(prepared.value)

    if (sequence !== compileSequenceRef.current) {
      return result
    }

    if (result.ok) {
      setCompiledGraph(result.descriptor)
      setRuntimeState("idle", null)
      return result
    }

    if (result.reason === "empty_graph") {
      setCompiledGraph(null)
      setRuntimeState("idle", result.detail)
      return result
    }

    setCompiledGraph(null)
    setRuntimeState(getRuntimeStatus(result), result.detail)
    return result
  }, [ensureTransformBinariesExistFunc, setCompiledGraph, setRuntimeState])
}
