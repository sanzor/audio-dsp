import { useCreatorPlaybackStore } from "@/Stores/CreatorPlaybackStore";
import { useListSources } from "@/hooks/sources/queries";

// Lets the creator pick what feeds the shared "Try it" playback session's
// input -- the original synthetic 440Hz test tone, or an uploaded
// Creator-surface signal source (see backend/domain/src/sources/source_info.rs).
// Lives in the status bar (not per-editor-surface) since CreatorPlaybackStore
// is one session shared across the primitive code-editor and the composite
// canvas -- see CreatorPlaybackStore.ts's file-level doc comment.
export function PlaybackInputSelector() {
  const { data: sources } = useListSources();
  const mode = useCreatorPlaybackStore((s) => s.playbackInputMode);
  const sourceId = useCreatorPlaybackStore((s) => s.playbackSourceId);
  const inputError = useCreatorPlaybackStore((s) => s.playbackInputError);
  const setPlaybackInputMode = useCreatorPlaybackStore((s) => s.setPlaybackInputMode);
  const setPlaybackInputSource = useCreatorPlaybackStore((s) => s.setPlaybackInputSource);

  const selectValue = mode === "source" && sourceId != null ? `source:${sourceId}` : "tone";

  function handleChange(value: string) {
    if (value === "tone") {
      setPlaybackInputMode("tone");
      return;
    }
    const id = Number(value.slice("source:".length));
    setPlaybackInputSource(id);
    setPlaybackInputMode("source");
  }

  return (
    <div className="flex items-center gap-1.5" title={inputError ? `Falling back to tone: ${inputError}` : undefined}>
      <span style={{ color: "var(--text-muted)" }}>INPUT:</span>
      <select
        value={selectValue}
        onChange={(e) => handleChange(e.target.value)}
        className="font-mono text-[10px] rounded px-1 py-0.5"
        style={{
          backgroundColor: "var(--bg-dark)",
          color: inputError ? "#ffd166" : "var(--text-main)",
          border: "1px solid rgba(255,255,255,0.12)",
          outline: "none",
        }}
      >
        <option value="tone">Tone (440Hz)</option>
        {sources?.map((s) => (
          <option key={s.source_id} value={`source:${s.source_id}`}>
            {s.source_info.name}
          </option>
        ))}
      </select>
      {inputError && <span style={{ color: "#ffd166" }}>⚠</span>}
    </div>
  );
}
