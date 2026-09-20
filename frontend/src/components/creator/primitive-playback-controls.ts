import { useCreatorPlaybackStore } from "@/Stores/CreatorPlaybackStore";
import { useCreatorStore } from "@/Stores/CreatorStore";
import { apiGetPrimitiveDraftBinary } from "@/Services/transform-drafts/TransformDraftsService";
import { useGetTransformDraft } from "@/hooks/transform-drafts/queries";
import { buildPrimitivePlaybackGraph, PRIMITIVE_PLAYBACK_NODE_ID } from "./creatorTransformPlayback";

// Extracted from code-editor.tsx's original inline play/stop toggle closure so
// the always-visible bottom playback stripe (playback-stripe.tsx) can drive
// the same "Try it" playback session for a primitive transform.
//
// Preview retrieves the binary from the current saved primitive draft. Save
// remains the only action that compiles and updates that binary; Publish is
// still an independent action.
export function usePrimitivePlaybackControls(transformId: number | null) {
  const playbackStatus = useCreatorPlaybackStore((s) => s.status);
  const playbackTransformId = useCreatorPlaybackStore((s) => s.playbackTransformId);
  const stopPlayback = useCreatorPlaybackStore((s) => s.stop);
  const editing = useCreatorStore((s) => s.editingTransformSource);
  const { data: draft } = useGetTransformDraft(transformId);
  const savedSourceIsOpen =
    editing?.transformId === transformId && editing.source === draft?.source_code;

  const isPlayingThis =
    transformId != null && playbackTransformId === transformId && playbackStatus !== "idle" && playbackStatus !== "error";
  const isLoading = transformId != null && playbackTransformId === transformId && playbackStatus === "loading";

  async function togglePlayback() {
    if (transformId == null) return;
    if (isPlayingThis) {
      stopPlayback();
      return;
    }
    if (draft == null || !draft.has_binary) return;
    const wasm = await apiGetPrimitiveDraftBinary(transformId);
    const graph = buildPrimitivePlaybackGraph([]);
    await useCreatorPlaybackStore.getState().play(
      transformId,
      draft.source_code ?? String(transformId),
      graph,
      { [PRIMITIVE_PLAYBACK_NODE_ID]: wasm },
      [],
    );
  }

  return {
    togglePlayback,
    isPlayingThis,
    isLoading,
    canStartPlayback: transformId != null && draft?.kind === "primitive" && draft.has_binary && savedSourceIsOpen,
    disabledReason: "Save a successfully compiled primitive draft before previewing it.",
  };
}
