import { useRef, useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { useListSources, useGetSourceAudio } from "@/hooks/sources/queries";
import { useUploadSource, useRenameSource, useDeleteSource } from "@/hooks/sources/mutations";
import { useCreatorPlaybackStore } from "@/Stores/CreatorPlaybackStore";
import type { SourceMeta } from "@/Services/SourceService";
import { encodeWav, interleave } from "@/audio/utils";

// Creator-surface signal-source management -- upload/list/rename/delete the
// real audio files selectable as the shared "Try it" playback session's
// input (see backend/domain/src/sources/source_info.rs and
// playback-input-selector.tsx, which handles day-to-day switching between
// sources once uploaded). Kept as its own local-state Dialog (not wired
// into the Editor's shared useUIStore.modalState) per the Editor/Creator
// separation the two surfaces otherwise maintain.

function formatDuration(seconds: number): string {
  if (!Number.isFinite(seconds) || seconds < 0) return "—";
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins}:${secs.toString().padStart(2, "0")}`;
}

interface SourceRowProps {
  source: SourceMeta;
  onSelectAsInput: (sourceId: number) => void;
}

function SourceRow({ source, onSelectAsInput }: SourceRowProps) {
  const [isRenaming, setIsRenaming] = useState(false);
  const [name, setName] = useState(source.source_info.name);
  const renameMutation = useRenameSource();
  const deleteMutation = useDeleteSource();
  const { objectUrl, isLoading } = useGetSourceAudio(isRenaming ? null : source.source_id);
  // ^ only fetches audio lazily once the row's audition control is actually
  // rendered/used below -- see the <audio> element, which mounts unconditionally,
  // so this really just avoids re-fetching while the row is mid-rename.

  const playbackInputMode = useCreatorPlaybackStore((s) => s.playbackInputMode);
  const playbackSourceId = useCreatorPlaybackStore((s) => s.playbackSourceId);
  const isActiveInput = playbackInputMode === "source" && playbackSourceId === source.source_id;

  function handleSaveRename() {
    const trimmed = name.trim();
    if (!trimmed || trimmed === source.source_info.name) {
      setIsRenaming(false);
      setName(source.source_info.name);
      return;
    }
    renameMutation.mutate(
      { source_id: source.source_id, source_name: trimmed },
      { onSuccess: () => setIsRenaming(false) }
    );
  }

  return (
    <div
      className="flex items-center gap-2 px-2 py-1.5 rounded text-xs"
      style={{ backgroundColor: "var(--bg-dark)", border: "1px solid rgba(255,255,255,0.05)" }}
    >
      {isRenaming ? (
        <Input
          autoFocus
          className="h-7 flex-1"
          value={name}
          onChange={(e) => setName(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter") handleSaveRename();
            if (e.key === "Escape") {
              setIsRenaming(false);
              setName(source.source_info.name);
            }
          }}
          onBlur={handleSaveRename}
        />
      ) : (
        <span className="flex-1 truncate" style={{ color: "var(--text-main)" }}>
          {source.source_info.name}
        </span>
      )}

      <span className="text-[10px] font-mono" style={{ color: "var(--text-muted)" }}>
        .{source.source_info.extension}
      </span>
      <span className="text-[10px] font-mono w-10 text-right" style={{ color: "var(--text-muted)" }}>
        {formatDuration(source.source_info.length)}
      </span>

      <audio controls preload="none" src={objectUrl ?? undefined} className="h-6" style={{ width: 140 }} />
      {isLoading && <span className="text-[9px]" style={{ color: "var(--text-muted)" }}>loading…</span>}

      <Button
        variant={isActiveInput ? "default" : "outline"}
        size="sm"
        className="h-7 text-[10px] px-2"
        onClick={() => onSelectAsInput(source.source_id)}
        title="Use as the Try it playback session's input"
      >
        {isActiveInput ? "Active input" : "Use as input"}
      </Button>
      {!isRenaming && (
        <Button variant="ghost" size="sm" className="h-7 px-2" onClick={() => setIsRenaming(true)}>
          Rename
        </Button>
      )}
      <Button
        variant="ghost"
        size="sm"
        className="h-7 px-2 text-destructive"
        disabled={deleteMutation.isPending}
        onClick={() => deleteMutation.mutate(source.source_id)}
      >
        Delete
      </Button>
    </div>
  );
}

interface SourcesPanelModalProps {
  open: boolean;
  onClose: () => void;
}

export function SourcesPanelModal({ open, onClose }: SourcesPanelModalProps) {
  const { data: sources, isLoading } = useListSources();
  const uploadMutation = useUploadSource();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [decodeError, setDecodeError] = useState<string | null>(null);
  const setPlaybackInputMode = useCreatorPlaybackStore((s) => s.setPlaybackInputMode);
  const setPlaybackInputSource = useCreatorPlaybackStore((s) => s.setPlaybackInputSource);

  // `/sources/add-source-multi` only accepts WAV bytes (RIFF/WAVE-sniffed) or
  // raw float32 PCM + sample_rate/channels -- it never decodes compressed
  // formats like MP3 server-side (see sources_controller.rs's endpoint doc).
  // So every picked file is decoded here via the Web Audio API and
  // re-encoded as WAV before upload, same as create-track-modal.tsx's
  // handleFileUpload -- this also normalizes any WAV variant the backend's
  // simple RIFF sniff might otherwise choke on.
  async function handleFilePicked(file: File) {
    const dotIndex = file.name.lastIndexOf(".");
    const name = dotIndex > 0 ? file.name.slice(0, dotIndex) : file.name;
    setDecodeError(null);

    const audioCtx = new AudioContext();
    let decoded: AudioBuffer;
    try {
      const arrayBuffer = await file.arrayBuffer();
      decoded = await audioCtx.decodeAudioData(arrayBuffer);
    } catch {
      setDecodeError(`Could not decode "${file.name}" as audio`);
      return;
    } finally {
      audioCtx.close();
    }

    const channelSamples = Array.from({ length: decoded.numberOfChannels }, (_, i) =>
      decoded.getChannelData(i)
    );
    const wavBlob = encodeWav(interleave(channelSamples), decoded.sampleRate, decoded.numberOfChannels);
    uploadMutation.mutate({ name, extension: "wav", fileBlob: wavBlob });
  }

  function handleSelectAsInput(sourceId: number) {
    setPlaybackInputSource(sourceId);
    setPlaybackInputMode("source");
  }

  return (
    <Dialog open={open} onOpenChange={(next) => !next && onClose()}>
      <DialogContent className="sm:max-w-2xl">
        <DialogHeader>
          <DialogTitle>Signal Sources</DialogTitle>
          <DialogDescription>
            Upload real audio files to audition against in the shared "Try it" playback session, as an
            alternative to the synthetic test tone.
          </DialogDescription>
        </DialogHeader>

        <div className="flex items-center gap-2 py-2">
          <input
            ref={fileInputRef}
            type="file"
            accept="audio/*,.wav,.mp3"
            className="hidden"
            onChange={(e) => {
              const file = e.target.files?.[0];
              if (file) handleFilePicked(file);
              e.target.value = "";
            }}
          />
          <Button onClick={() => fileInputRef.current?.click()} disabled={uploadMutation.isPending}>
            {uploadMutation.isPending ? "Uploading…" : "Upload audio file"}
          </Button>
          {decodeError && <span className="text-xs text-destructive">{decodeError}</span>}
          {uploadMutation.isError && (
            <span className="text-xs text-destructive">
              {uploadMutation.error instanceof Error ? uploadMutation.error.message : "Upload failed"}
            </span>
          )}
        </div>

        <div className="flex flex-col gap-1.5 max-h-80 overflow-y-auto">
          {isLoading && (
            <span className="text-xs" style={{ color: "var(--text-muted)" }}>
              Loading sources…
            </span>
          )}
          {!isLoading && (sources == null || sources.length === 0) && (
            <span className="text-xs" style={{ color: "var(--text-muted)" }}>
              No sources uploaded yet.
            </span>
          )}
          {sources?.map((source) => (
            <SourceRow key={source.source_id} source={source} onSelectAsInput={handleSelectAsInput} />
          ))}
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={onClose}>
            Close
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
