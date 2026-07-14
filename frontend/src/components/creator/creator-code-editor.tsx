import { useState } from "react";
import Editor from "@monaco-editor/react";
import { useCreatorStore } from "@/Stores/CreatorStore";
import { useCompileTicketStatus, useGetTransformDefinition } from "@/hooks/transforms/queries";
import { useRequestCompileTransform } from "@/hooks/transforms/mutations";

const DEFAULT_CODE = `use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RMSDetector {
    window_size: usize,
    buffer: Vec<f32>,
}

impl TransformNode for RMSDetector {
    fn process(&mut self, input: &[f32]) -> f32 {
        let sum_sq: f32 = input.iter().map(|&x| x * x).sum();
        (sum_sq / input.len() as f32).sqrt()
    }
}
`;

interface FileTab {
  id: string;
  name: string;
  language: string;
}

const CONFIG_TAB: FileTab = { id: "config", name: "config.json", language: "json" };

export function CreatorCodeEditor() {
  const selectedId = useCreatorStore((s) => s.selectedTransformId);
  const activeTicketByTransform = useCreatorStore((s) => s.activeTicketByTransform);
  const setActiveTicket = useCreatorStore((s) => s.setActiveTicket);
  const { data: definition } = useGetTransformDefinition(selectedId);
  const [activeTab, setActiveTab] = useState("impl");
  const [code, setCode] = useState(DEFAULT_CODE);

  const ticketId = selectedId != null ? activeTicketByTransform[selectedId] ?? null : null;
  const compileMutation = useRequestCompileTransform();
  const ticketStatus = useCompileTicketStatus(ticketId, selectedId);

  const buildState = ticketStatus.data?.status.state ?? null;
  const isCompiling = compileMutation.isPending || buildState === "processing";

  function handleCompile() {
    if (selectedId == null || !code.trim() || isCompiling) return;
    compileMutation.mutate(
      { transform_id: selectedId, source_code: code },
      { onSuccess: (ticket) => setActiveTicket(selectedId, ticket.ticket_id) }
    );
  }

  const tabs: FileTab[] = [
    { id: "impl", name: definition ? `${definition.name}.rs` : "untitled.rs", language: "rust" },
    CONFIG_TAB,
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
          {buildState === "failed" && (
            <span
              className="font-mono text-[10px] max-w-[280px] truncate"
              title={ticketStatus.data?.status.message}
              style={{ color: "#ff6b6b" }}
            >
              Failed{ticketStatus.data?.status.message ? `: ${ticketStatus.data.status.message}` : ""}
            </span>
          )}
          <button
            onClick={handleCompile}
            disabled={selectedId == null || !code.trim() || isCompiling}
            className="font-mono font-bold px-2.5 py-0.5 rounded text-[10px] transition-colors"
            style={{
              color: "#adc6ff",
              border: "1px solid rgba(173,198,255,0.4)",
              opacity: selectedId == null || !code.trim() || isCompiling ? 0.5 : 1,
            }}
          >
            Compile
          </button>
          <span
            className="font-mono font-bold px-2 py-0.5 rounded text-[10px]"
            style={{ color: "#ffb786", border: "1px solid rgba(255,183,134,0.3)" }}
          >
            RUST (WASM)
          </span>
        </div>
      </div>

      {/* Monaco editor */}
      <div className="flex-1 min-h-0">
        <Editor
          height="100%"
          language={tabs.find((t) => t.id === activeTab)?.language ?? "rust"}
          value={code}
          onChange={(value) => setCode(value ?? "")}
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
      </div>
    </div>
  );
}
