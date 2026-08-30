import { useCreatorStore } from "@/Stores/CreatorStore";
import { useCreatorPlaybackStore } from "@/Stores/CreatorPlaybackStore";
import { decodeBase64Binary } from "@/Services/TransformService";
import { buildPrimitivePlaybackGraph, PRIMITIVE_PLAYBACK_NODE_ID } from "./creatorTransformPlayback";

// Extracted from code-editor.tsx's original inline play/stop toggle closure so
// the always-visible bottom playback stripe (playback-stripe.tsx) can drive
// the same "Try it" playback session for a primitive transform, without
// duplicating the resource-attach / wasm-decode logic. code-editor.tsx no
// longer starts playback itself (its own inline Play/Stop button was
// removed in favor of the stripe) -- it still computes `isPlayingThis`
// locally (trivial, store-only) for its Bypass toggle, which doesn't need
// this hook. Sibling of composite/composite-playback-controls.ts's
// useCompositePlaybackControls, same "compile/resolve inputs, then hand off
// to CreatorPlaybackStore.play" shape, but for a single already-compiled
// primitive resource instead of a graph.
export function usePrimitivePlaybackControls(transformId: number | null) {
  const editing = useCreatorStore((s) => s.editingTransformSource);
  const compiledDraftByTransform = useCreatorStore((s) => s.compiledDraftByTransform);

  const playbackStatus = useCreatorPlaybackStore((s) => s.status);
  const playbackTransformId = useCreatorPlaybackStore((s) => s.playbackTransformId);
  const startPlayback = useCreatorPlaybackStore((s) => s.play);
  const stopPlayback = useCreatorPlaybackStore((s) => s.stop);

  const code = editing?.transformId === transformId ? editing.source : "";
  const attachableCompiledDraft =
    transformId != null && compiledDraftByTransform[transformId]?.sourceCode === code
      ? compiledDraftByTransform[transformId]
      : undefined;

  const isPlayingThis =
    transformId != null && playbackTransformId === transformId && playbackStatus !== "idle" && playbackStatus !== "error";
  const isLoading = transformId != null && playbackTransformId === transformId && playbackStatus === "loading";
  const canStartPlayback = attachableCompiledDraft != null;

  function togglePlayback() {
    if (transformId == null) return;
    if (isPlayingThis) {
      stopPlayback();
      return;
    }
    if (attachableCompiledDraft == null) return;
    const wasmBytes = decodeBase64Binary(attachableCompiledDraft.wasmBase64);
    const params: number[] = [];
    const resourceKey = `${transformId}:${code}`;
    const graph = buildPrimitivePlaybackGraph(params);
    void startPlayback(transformId, resourceKey, graph, { [PRIMITIVE_PLAYBACK_NODE_ID]: wasmBytes }, params);
  }

  return { togglePlayback, isPlayingThis, isLoading, canStartPlayback };
}
