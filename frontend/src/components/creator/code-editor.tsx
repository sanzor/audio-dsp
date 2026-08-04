import { useEffect, useMemo, useState } from "react";
import Editor from "@monaco-editor/react";
import { useCreatorStore } from "@/Stores/CreatorStore";
import { useCreatorPlaybackStore } from "@/Stores/CreatorPlaybackStore";
import { useGetTransformDefinition } from "@/hooks/transforms/queries";
import { useCompileTicketStatus } from "@/hooks/tickets/queries";
import { useRequestCompileTransform } from "@/hooks/tickets/mutations";
import { useSaveTransform, usePublishTransform } from "@/hooks/transforms/mutations";
import { validateTransformSource } from "./validateTransformSource";
import { usePublishWithPortShapeDiff } from "./usePublishWithPortShapeDiff";
import { ToolbarButton } from "./toolbar-button";

// The "Try it" client-side playback (creatorTransformPlayback.ts) runs a
// just-compiled binary through CreatorPlaybackStore, fed by the wasm_base64/
// params fields on the compile ticket status DTO — see
// agents/decisions/0003-transform-preview-flow.md.

const DEFAULT_CODE = `use transform_sdk::{Transform, TransformMetadata, PortMetadata, ParamMetadata, Direction, PortKind, PortCardinality, Params};

#[derive(Default)]
pub struct RmsDetector {
    window_size: usize,
}

impl Transform for RmsDetector {
    // \`samples\` has one entry per declared input port, in order — a
    // single-input transform like this one reads samples[0], same as always.
    fn process(&mut self, samples: &[&[f32]], _params: &Params<'_>) -> Vec<f32> {
        let input = samples[0];
        let sum_sq: f32 = input.iter().map(|&x| x * x).sum();
        let rms = (sum_sq / input.len() as f32).sqrt();
        vec![rms; input.len()]
    }

    fn metadata() -> TransformMetadata {
        TransformMetadata {
            name: "RMS Detector".to_string(),
            description: Some("Replaces each sample with the block's RMS level.".to_string()),
            ports: vec![
                PortMetadata { name: "in".to_string(), direction: Direction::Input, order: 0, description: None, kind: PortKind::Program, cardinality: PortCardinality::Single },
                PortMetadata { name: "out".to_string(), direction: Direction::Output, order: 0, description: None, kind: PortKind::Program, cardinality: PortCardinality::Single },
            ],
            params: vec![],
        }
    }
}

transform_sdk::export_transform!(RmsDetector);
`;

interface FileTab {
  id: string;
  name: string;
  language: string;
}

const OUTPUT_TAB: FileTab = { id: "output", name: "output", language: "text" };

export function CreatorCodeEditor() {
  const selectedId = useCreatorStore((s) => s.selectedTransformId);
  const activeTicketByTransform = useCreatorStore((s) => s.activeTicketByTransform);
  const setActiveTicket = useCreatorStore((s) => s.setActiveTicket);
  const lastCompiledResourceByTransform = useCreatorStore((s) => s.lastCompiledResourceByTransform);
  const setLastCompiledResource = useCreatorStore((s) => s.setLastCompiledResource);
  const editing = useCreatorStore((s) => s.editingTransformSource);
  const beginEditingTransformSource = useCreatorStore((s) => s.beginEditingTransformSource);
  const updateEditingTransformSource = useCreatorStore((s) => s.updateEditingTransformSource);
  const markTransformSourceSaved = useCreatorStore((s) => s.markTransformSourceSaved);
  const { data: definition } = useGetTransformDefinition(selectedId);
  const [activeTab, setActiveTab] = useState("impl");

  const playbackStatus = useCreatorPlaybackStore((s) => s.status);
  const playbackTransformId = useCreatorPlaybackStore((s) => s.playbackTransformId);
  const playbackBypassed = useCreatorPlaybackStore((s) => s.bypassed);
  const setPlaybackBypass = useCreatorPlaybackStore((s) => s.setBypass);

  // Load the editor buffer from the fetched definition once per selected
  // transform — not on every render, and not before the right definition
  // has actually arrived (avoids briefly showing the previous transform's code).
  const editingForSelected = editing?.transformId === selectedId ? editing : null;
  useEffect(() => {
    if (selectedId == null || editingForSelected != null) return;
    if (definition == null || definition.transform_id !== selectedId) return;
    beginEditingTransformSource(selectedId, definition.source_code || DEFAULT_CODE);
  }, [selectedId, definition, editingForSelected, beginEditingTransformSource]);

  const code = editingForSelected?.source ?? "";
  const isDirty = editingForSelected != null && editingForSelected.source !== editingForSelected.originalSource;

  const activeTicket = selectedId != null ? activeTicketByTransform[selectedId] ?? null : null;
  const compileMutation = useRequestCompileTransform();
  const saveMutation = useSaveTransform(selectedId ?? -1);
  const publishMutation = usePublishTransform(selectedId ?? -1);
  const ticketStatus = useCompileTicketStatus(activeTicket?.ticketId ?? null, selectedId);

  const buildState = ticketStatus.data?.status.state ?? null;
  const buildMessage = ticketStatus.data?.status.message ?? null;
  const isCompiling = compileMutation.isPending || buildState === "processing";
  const resourceId = ticketStatus.data?.status.resource_id ?? null;

  // Once a compile ticket succeeds, record its resource against the exact
  // source that produced it — Save can only safely attach it while the
  // buffer still matches that text.
  useEffect(() => {
    if (selectedId == null || resourceId == null || activeTicket == null) return;
    if (lastCompiledResourceByTransform[selectedId]?.resourceId === resourceId) return;
    setLastCompiledResource(selectedId, resourceId, activeTicket.sourceCode);
  }, [selectedId, resourceId, activeTicket, lastCompiledResourceByTransform, setLastCompiledResource]);

  const attachableResourceId =
    selectedId != null && lastCompiledResourceByTransform[selectedId]?.sourceCode === code
      ? lastCompiledResourceByTransform[selectedId].resourceId
      : undefined;

  const validation = useMemo(() => validateTransformSource(code), [code]);

  // Tear down any live playback session whenever the selected transform
  // changes (or this editor unmounts) — switching transforms must never
  // leave the previous one's audio session running.
  useEffect(() => {
    return () => {
      useCreatorPlaybackStore.getState().stop();
    };
  }, [selectedId]);

  const isPlayingThis =
    selectedId != null && playbackTransformId === selectedId && playbackStatus !== "idle" && playbackStatus !== "error";

  // Composite transforms never reach this component — creator-workspace.tsx
  // routes kind === "composite" to CompositeCanvas instead, which has its
  // own graph-based Monaco-free authoring flow.
  //
  // This component's own inline Play/Stop button was removed in favor of the
  // always-visible bottom playback stripe (playback-stripe.tsx), which drives
  // the same playback session via primitive-playback-controls.ts's
  // usePrimitivePlaybackControls -- the extracted equivalent of what used to
  // be this component's local play/stop toggle closure. isPlayingThis
  // above is still computed locally since the Bypass toggle below still
  // needs it, and that's a trivial store-only read.

  function handleSave() {
    if (selectedId == null || !isDirty) return;
    const source = code;
    saveMutation.mutate(
      { source_code: source, resource_id: attachableResourceId },
      { onSuccess: () => markTransformSourceSaved(selectedId, source) }
    );
  }

  function handleCompile() {
    if (selectedId == null || !validation.ok || isCompiling) return;
    compileMutation.mutate(
      { transform_id: selectedId, source_code: code },
      { onSuccess: (ticket) => setActiveTicket(selectedId, ticket.ticket_id, code) }
    );
  }

  const { handlePublish } = usePublishWithPortShapeDiff(selectedId, publishMutation);

  const tabs: FileTab[] = [
    { id: "impl", name: definition ? `${definition.name}.rs` : "untitled.rs", language: "rust" },
    ...(buildState === "failed" ? [OUTPUT_TAB] : []),
  ];

  return (
    <div
      className="flex flex-col h-full"
      style={{ backgroundColor: "#1a1a1a", borderTop: "1px solid rgba(255,255,255,0.06)" }}
    >
      {/* Tab bar */}
      <div
        className="flex items-center justify-between px-3 h-8 flex-shrink-0"
        style={{ borderBottom: "1px solid rgba(255,255,255,0.06)", backgroundColor: "var(--bg-darker)" }}
      >
        <div className="flex items-center">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className="flex items-center gap-1.5 h-8 px-3 text-[11px] font-mono transition-colors"
              style={
                activeTab === tab.id
                  ? {
                      color: "#adc6ff",
                      borderBottom: "2px solid #adc6ff",
                      backgroundColor: "rgba(173,198,255,0.06)",
                    }
                  : { color: "var(--text-muted)", opacity: 0.6 }
              }
            >
              {tab.name}
            </button>
          ))}
        </div>
        <div className="flex items-center gap-2">
          {!validation.ok && !isCompiling && buildState !== "failed" && (
            <span
              className="font-mono text-[10px] max-w-[280px] truncate"
              title={validation.issues.join(" ")}
              style={{ color: "#ffd166" }}
            >
              {validation.issues[0]}
            </span>
          )}
          {buildState === "processing" && (
            <span className="font-mono text-[10px]" style={{ color: "#ffd166" }}>
              Compiling…
            </span>
          )}
          {buildState === "successful" && (
            <span className="font-mono text-[10px]" style={{ color: "#4ae176" }}>
              Compiled ✓{ticketStatus.data?.status.resource_id != null ? ` (resource #${ticketStatus.data.status.resource_id})` : ""}
            </span>
          )}
          {buildState === "successful" && isPlayingThis && (
            <ToolbarButton
              variant="bypass"
              onClick={() => setPlaybackBypass(!playbackBypassed)}
              muted={playbackBypassed}
            >
              {playbackBypassed ? "Bypassed" : "Bypass"}
            </ToolbarButton>
          )}
          {buildState === "failed" && (
            <button
              onClick={() => setActiveTab("output")}
              className="font-mono text-[10px] max-w-[280px] truncate"
              title="View full compiler output"
              style={{ color: "#ff6b6b", background: "none", border: "none", padding: 0, cursor: "pointer" }}
            >
              Failed{buildMessage ? `: ${buildMessage}` : ""}
            </button>
          )}
          <ToolbarButton
            variant="save"
            onClick={handleSave}
            disabled={selectedId == null || !isDirty || saveMutation.isPending}
          >
            {saveMutation.isPending ? "Saving…" : "Save Draft"}
          </ToolbarButton>
          <ToolbarButton
            variant="compile"
            onClick={handleCompile}
            disabled={selectedId == null || !validation.ok || isCompiling}
            title={!validation.ok ? validation.issues.join(" ") : undefined}
          >
            Compile
          </ToolbarButton>
          <ToolbarButton
            variant="publish"
            onClick={handlePublish}
            disabled={selectedId == null || publishMutation.isPending}
            title={publishMutation.error?.message}
          >
            {publishMutation.isPending ? "Publishing…" : "Publish"}
          </ToolbarButton>
          {publishMutation.isError && (
            <span
              className="font-mono text-[10px] max-w-[240px] truncate"
              title={publishMutation.error.message}
              style={{ color: "#ff6b6b" }}
            >
              {publishMutation.error.message}
            </span>
          )}
          <span
            className="font-mono font-bold px-2 py-0.5 rounded text-[10px]"
            style={{ color: "#ffb786", border: "1px solid rgba(255,183,134,0.3)" }}
          >
            RUST (WASM)
          </span>
        </div>
      </div>

      {/* Monaco editor / output panel */}
      <div className="flex-1 min-h-0">
        {activeTab === "output" ? (
          <pre
            className="w-full h-full overflow-auto m-0 p-3 text-[11px] font-mono whitespace-pre-wrap"
            style={{ color: "#ff8a8a" }}
          >
            {buildMessage ?? "No output."}
          </pre>
        ) : (
          <Editor
            height="100%"
            language={tabs.find((t) => t.id === activeTab)?.language ?? "rust"}
            value={code}
            onChange={(value) => updateEditingTransformSource(value ?? "")}
            theme="vs-dark"
            options={{
              fontSize: 13,
              fontFamily: "JetBrains Mono, monospace",
              minimap: { enabled: false },
              lineNumbers: "on",
              scrollBeyondLastLine: false,
              renderLineHighlight: "line",
              padding: { top: 8, bottom: 8 },
              scrollbar: { verticalScrollbarSize: 4, horizontalScrollbarSize: 4 },
            }}
          />
        )}
      </div>
    </div>
  );
}
