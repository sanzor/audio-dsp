import { useCreatorStore } from "@/Stores/CreatorStore";
import { useCreatorPlaybackStore } from "@/Stores/CreatorPlaybackStore";
import { useCompileTicketStatus } from "@/hooks/tickets/queries";
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
  const activeTicketByTransform = useCreatorStore((s) => s.activeTicketByTransform);
  const editing = useCreatorStore((s) => s.editingTransformSource);
  const lastCompiledResourceByTransform = useCreatorStore((s) => s.lastCompiledResourceByTransform);

  const playbackStatus = useCreatorPlaybackStore((s) => s.status);
  const playbackTransformId = useCreatorPlaybackStore((s) => s.playbackTransformId);
  const startPlayback = useCreatorPlaybackStore((s) => s.play);
  const stopPlayback = useCreatorPlaybackStore((s) => s.stop);

  const activeTicket = transformId != null ? activeTicketByTransform[transformId] ?? null : null;
  const ticketStatus = useCompileTicketStatus(activeTicket?.ticketId ?? null, transformId);

  const code = editing?.transformId === transformId ? editing.source : "";
  const attachableResourceId =
    transformId != null && lastCompiledResourceByTransform[transformId]?.sourceCode === code
      ? lastCompiledResourceByTransform[transformId].resourceId
      : undefined;

  const isPlayingThis =
    transformId != null && playbackTransformId === transformId && playbackStatus !== "idle" && playbackStatus !== "error";
  const isLoading = transformId != null && playbackTransformId === transformId && playbackStatus === "loading";
  const canStartPlayback = attachableResourceId != null && ticketStatus.data?.status.wasm_base64 != null;

  function togglePlayback() {
    if (transformId == null) return;
    if (isPlayingThis) {
      stopPlayback();
      return;
    }
    const wasmBase64 = ticketStatus.data?.status.wasm_base64;
    if (wasmBase64 == null || attachableResourceId == null) return;
    const wasmBytes = decodeBase64Binary(wasmBase64);
    const params = [...(ticketStatus.data?.status.params ?? [])]
      .sort((a, b) => a.param_order - b.param_order)
      .map((p) => p.default_value);
    const resourceKey = `${attachableResourceId}:${code}`;
    const graph = buildPrimitivePlaybackGraph(params);
    void startPlayback(transformId, resourceKey, graph, { [PRIMITIVE_PLAYBACK_NODE_ID]: wasmBytes }, params);
  }

  return { togglePlayback, isPlayingThis, isLoading, canStartPlayback };
}
