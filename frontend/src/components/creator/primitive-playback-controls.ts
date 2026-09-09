import { useCreatorPlaybackStore } from "@/Stores/CreatorPlaybackStore";

// Extracted from code-editor.tsx's original inline play/stop toggle closure so
// the always-visible bottom playback stripe (playback-stripe.tsx) can drive
// the same "Try it" playback session for a primitive transform.
//
// As of the transform-draft API reshape (see
// agents/decisions/0011-transform-draft-api-reshape.md), there is no data
// source left for this: the ticket-compiled wasm this used to run
// (compiledDraftByTransform) no longer exists, Save's response carries no
// wasm bytes (TransformDraftDto only has `has_binary: bool`), and there is
// no backend route to fetch a draft's own not-yet-published wasm either.
// "Try it" for a primitive is disabled outright rather than silently
// broken — see this pass's final report for the backend gap this implies
// (a route to fetch a draft's own compiled-but-unpublished binary would be
// needed to bring this back). Composite "Try it" is unaffected — it runs
// entirely client-side via GraphCompiler, not this hook (see
// composite/composite-playback-controls.ts).
export function usePrimitivePlaybackControls(transformId: number | null) {
  const playbackStatus = useCreatorPlaybackStore((s) => s.status);
  const playbackTransformId = useCreatorPlaybackStore((s) => s.playbackTransformId);
  const stopPlayback = useCreatorPlaybackStore((s) => s.stop);

  const isPlayingThis =
    transformId != null && playbackTransformId === transformId && playbackStatus !== "idle" && playbackStatus !== "error";
  const isLoading = transformId != null && playbackTransformId === transformId && playbackStatus === "loading";

  function togglePlayback() {
    if (transformId == null) return;
    if (isPlayingThis) {
      stopPlayback();
    }
    // No else branch: starting playback has no data source anymore (see
    // module comment above) — canStartPlayback is always false, so
    // playback-stripe.tsx never calls this to start, only to stop.
  }

  return {
    togglePlayback,
    isPlayingThis,
    isLoading,
    canStartPlayback: false,
    disabledReason: "Preview isn't available for unpublished/unsaved-since-publish primitives yet — there's no way to fetch a draft's compiled binary before it's published.",
  };
}
