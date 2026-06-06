import { useState } from "react";
import Editor from "@monaco-editor/react";

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

const TABS: FileTab[] = [
  { id: "impl", name: "RMS_DETECTOR.rs", language: "rust" },
  { id: "config", name: "config.json", language: "json" },
];

export function CreatorCodeEditor() {
  const [activeTab, setActiveTab] = useState("impl");
  const [code, setCode] = useState(DEFAULT_CODE);

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
          {TABS.map((tab) => (
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
        <span
          className="font-mono font-bold px-2 py-0.5 rounded text-[10px]"
          style={{ color: "#ffb786", border: "1px solid rgba(255,183,134,0.3)" }}
        >
          RUST (WASM)
        </span>
      </div>

      {/* Monaco editor */}
      <div className="flex-1 min-h-0">
        <Editor
          height="100%"
          language={TABS.find((t) => t.id === activeTab)?.language ?? "rust"}
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
