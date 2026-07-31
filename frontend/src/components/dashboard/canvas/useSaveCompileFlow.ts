import { useCallback, useEffect, useRef, useState } from "react";
import type { Node, Edge } from "reactflow";
import type { CompileGraphResult } from "@/audio/pipeline/runPipeline";
import { useAudioEffectsStore } from "@/Stores/AudioEffectsStore";
import type { SaveCompileStatusState } from "./save-compile-status-overlay";

const COMPILE_AFTER_SAVE_TIMEOUT_MS = 8000;

interface UseSaveCompileFlowParams {
  graphId: number | undefined;
  nodes: Node[];
  edges: Edge[];
  handleSaveGraph: (graphId: number, nodes: Node[], edges: Edge[]) => Promise<void>;
  compileNow: (nodesOverride?: Node[]) => CompileGraphResult;
  compiled: boolean;
}

export function useSaveCompileFlow({
  graphId,
  nodes,
  edges,
  handleSaveGraph,
  compileNow,
  compiled,
}: UseSaveCompileFlowParams): {
  saveState: "hidden" | "saving" | "success" | "error";
  saveProgress: number;
  saveCompileState: SaveCompileStatusState;
  saveCompileMessage: string | null;
  handleSave: (nodesOverride?: Node[]) => Promise<void>;
} {
  const [saveProgress, setSaveProgress] = useState(0);
  const [saveState, setSaveState] = useState<"hidden" | "saving" | "success" | "error">("hidden");
  const [saveCompileState, setSaveCompileState] = useState<SaveCompileStatusState>("hidden");
  const [saveCompileMessage, setSaveCompileMessage] = useState<string | null>(null);
  const saveProgressTimerRef = useRef<number | null>(null);
  const saveHideTimerRef = useRef<number | null>(null);
  const compileWaitTimerRef = useRef<number | null>(null);
  const awaitingCompileAfterSaveRef = useRef(false);

  const { runtimeStatus, runtimeMessage } = useAudioEffectsStore();

  const clearSaveTimers = useCallback(() => {
    if (saveProgressTimerRef.current != null) {
      window.clearInterval(saveProgressTimerRef.current);
      saveProgressTimerRef.current = null;
    }
    if (saveHideTimerRef.current != null) {
      window.clearTimeout(saveHideTimerRef.current);
      saveHideTimerRef.current = null;
    }
  }, []);

  const clearCompileWaitTimer = useCallback(() => {
    if (compileWaitTimerRef.current != null) {
      window.clearTimeout(compileWaitTimerRef.current);
      compileWaitTimerRef.current = null;
    }
  }, []);

  useEffect(() => clearSaveTimers, [clearSaveTimers]);
  useEffect(() => clearCompileWaitTimer, [clearCompileWaitTimer]);

  useEffect(() => {
    if (!awaitingCompileAfterSaveRef.current) {
      if (saveCompileState === "error" && compiled) {
        setSaveCompileState("hidden");
        setSaveCompileMessage(null);
      }
      return;
    }

    if (compiled) {
      awaitingCompileAfterSaveRef.current = false;
      clearCompileWaitTimer();
      setSaveCompileState("hidden");
      setSaveCompileMessage(null);
      return;
    }

    if (runtimeStatus === "hydrating") {
      setSaveCompileState("waiting");
      setSaveCompileMessage(runtimeMessage ?? "Fetching transform binaries...");
      return;
    }

    if (runtimeStatus === "error") {
      awaitingCompileAfterSaveRef.current = false;
      clearCompileWaitTimer();
      setSaveCompileState("error");
      setSaveCompileMessage(runtimeMessage ?? "Compile failed after save.");
      return;
    }

    if (runtimeStatus === "idle") {
      awaitingCompileAfterSaveRef.current = false;
      clearCompileWaitTimer();
      setSaveCompileState("error");
      setSaveCompileMessage(runtimeMessage ?? "Compile did not produce a runnable graph.");
    }
  }, [
    clearCompileWaitTimer,
    compiled,
    runtimeMessage,
    runtimeStatus,
    saveCompileState,
  ]);

  const handleSave = useCallback(async (nodesOverride?: Node[]) => {
    if (graphId == null || saveState === "saving") return;

    clearSaveTimers();
    setSaveState("saving");
    setSaveProgress(8);
    saveProgressTimerRef.current = window.setInterval(() => {
      setSaveProgress((current) => {
        const remaining = 94 - current;
        if (remaining <= 0) return current;
        return current + Math.max(1, remaining * 0.18);
      });
    }, 140);

    try {
      await handleSaveGraph(graphId, nodesOverride ?? nodes, edges);
      awaitingCompileAfterSaveRef.current = true;
      clearCompileWaitTimer();
      setSaveCompileState("waiting");
      setSaveCompileMessage("Checking saved graph...");
      compileNow(nodesOverride);
      compileWaitTimerRef.current = window.setTimeout(() => {
        if (!awaitingCompileAfterSaveRef.current) return;
        awaitingCompileAfterSaveRef.current = false;
        setSaveCompileState("error");
        setSaveCompileMessage("Compile timed out while waiting for transform binaries.");
      }, COMPILE_AFTER_SAVE_TIMEOUT_MS);
      clearSaveTimers();
      setSaveState("success");
      setSaveProgress(100);
      saveHideTimerRef.current = window.setTimeout(() => {
        setSaveState("hidden");
        setSaveProgress(0);
        saveHideTimerRef.current = null;
      }, 550);
    } catch (error) {
      clearSaveTimers();
      setSaveState("error");
      setSaveProgress(100);
      saveHideTimerRef.current = window.setTimeout(() => {
        setSaveState("hidden");
        setSaveProgress(0);
        saveHideTimerRef.current = null;
      }, 1800);
      throw error;
    }
  }, [graphId, nodes, edges, handleSaveGraph, saveState, clearCompileWaitTimer, clearSaveTimers, compileNow]);

  return { saveState, saveProgress, saveCompileState, saveCompileMessage, handleSave };
}
