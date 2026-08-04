import { useCreatorStore } from "@/Stores/CreatorStore";
import { useCreatorPlaybackStore } from "@/Stores/CreatorPlaybackStore";
import { useCompositeCanvasStore } from "@/Stores/CompositeCanvasStore";
import { useGetTransformDefinition } from "@/hooks/transforms/queries";
import { useCompositePlaybackControls } from "./composite/composite-playback-controls";
import { usePrimitivePlaybackControls } from "./primitive-playback-controls";
import { PlaybackWaveform } from "./playback-waveform";

// Always-visible bottom playback stripe, sitting between CreatorWorkspace
// and CreatorStatusBar (see index.tsx). Replaces the waveform popover that
// used to hang off CreatorStatusBar's "WAVEFORM ▸/▾" toggle, and replaces
// the top-toolbar Play/Stop buttons removed from composite-canvas.tsx and
// code-editor.tsx -- this is now the one place Play/Stop lives for both
// surfaces.
//
// Same visibility gate the old popover used: only rendered in "source" input
// mode with a source selected (collapses to nothing otherwise, e.g. tone
// mode). Which engine Play/Stop drives depends on which surface is
// currently open -- composite (useCompositePlaybackControls, the same
// compile-and-play path composite-canvas.tsx's removed button used) or
// primitive (usePrimitivePlaybackControls, extracted from code-editor.tsx's
// former local play/stop toggle closure). Both hooks are called unconditionally (rules
// of hooks); only the one matching the open transform's kind is ever acted
// on. Stop is engine-agnostic -- it always just tears down whatever
// CreatorPlaybackStore session is running, regardless of which surface
// started it.
export function PlaybackStripe() {
  const playbackInputMode = useCreatorPlaybackStore((s) => s.playbackInputMode);
  const playbackSourceId = useCreatorPlaybackStore((s) => s.playbackSourceId);
  const playbackStatus = useCreatorPlaybackStore((s) => s.status);
  const playbackTransformId = useCreatorPlaybackStore((s) => s.playbackTransformId);
  const stopPlayback = useCreatorPlaybackStore((s) => s.stop);

  const selectedId = useCreatorStore((s) => s.selectedTransformId);
  const { data: definition } = useGetTransformDefinition(selectedId);
  const activeKind = definition?.transform_id === selectedId ? definition?.kind ?? null : null;

  const editingGraph = useCompositeCanvasStore((s) => s.editingGraph);
  const compositeControls = useCompositePlaybackControls(selectedId ?? -1);
  const primitiveControls = usePrimitivePlaybackControls(selectedId);

  const canShow = playbackInputMode === "source" && playbackSourceId != null;
  if (!canShow) return null;

  const isPlaying =
    selectedId != null && playbackTransformId === selectedId && playbackStatus !== "idle" && playbackStatus !== "error";

  const canPlay =
    activeKind === "composite"
      ? editingGraph != null && editingGraph.transformId === selectedId && editingGraph.nodes.size > 0
      : activeKind === "primitive"
        ? primitiveControls.canStartPlayback
        : false;

  function handlePlay() {
    if (activeKind === "composite") compositeControls.togglePlayback();
    else if (activeKind === "primitive") primitiveControls.togglePlayback();
  }

  function handleStop() {
    stopPlayback();
  }

  return (
    <div
      className="flex items-center h-12 flex-shrink-0"
      style={{ backgroundColor: "var(--bg-darker)", borderTop: "1px solid rgba(255,255,255,0.06)" }}
    >
      <PlaybackWaveform isPlaying={isPlaying} playDisabled={!canPlay} onPlay={handlePlay} onStop={handleStop} />
    </div>
  );
}
