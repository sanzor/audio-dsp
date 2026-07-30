import { useState } from "react";
import Editor from "@monaco-editor/react";
import { useTransformStore } from "@/Stores/TransformStore";
import { useCreatorPreviewStore } from "@/Stores/CreatorPreviewStore";
import { ParamRow, PortsList } from "../transform-properties-panel";

// ─── Node inspector (bottom panel) ─────────────────────────────────────────────
// Phase 1 + 2 of agents/decisions/0005-composite-node-inspector.md. Replaces
// the old openModal({type: "transformDetails"}) call on node click — this is
// the only call site that changes; TransformDetailsModal and its other call
// site (TransformItem.tsx) are untouched. Two Monaco-tab-bar-style tabs,
// matching code-editor.tsx's impl.rs/output pattern:
//   - Source: read-only Monaco of the selected node's transform source —
//     already resolved client-side via useResolveTransformDefinitions, no
//     new fetch.
//   - Details: ports/params, via the same prop-driven PortsList/ParamRow
//     transform-properties-panel.tsx exports, driven by this node instead of
//     useCreatorStore.selectedTransformId. Params are ephemeral-live-editable
//     while this composite is previewing and this node is present in the
//     currently compiled preview graph (Phase 2) — never written into
//     editingGraph, never part of Save/Publish.

type InspectorTab = "source" | "details";

interface NodeInspectorPanelProps {
  nodeId: number;
  transformId: number;
  compositeTransformId: number;
  onClose: () => void;
}

export function NodeInspectorPanel({ nodeId, transformId, compositeTransformId, onClose }: NodeInspectorPanelProps) {
  const [activeTab, setActiveTab] = useState<InspectorTab>("source");
  const definition = useTransformStore((s) => s.definitions.get(transformId));

  const previewStatus = useCreatorPreviewStore((s) => s.status);
  const previewTransformId = useCreatorPreviewStore((s) => s.previewTransformId);
  const previewGraph = useCreatorPreviewStore((s) => s.previewGraph);
  const nodeParamValues = useCreatorPreviewStore((s) => s.nodeParamValues);
  const updateNodeParam = useCreatorPreviewStore((s) => s.updateNodeParam);

  // "Is this node actually present in the compiled graph currently loaded
  // into the worklet" — not just "is this composite previewing at all",
  // since Phase 3's disabled-node filtering can drop this exact node out of
  // the compiled graph while the composite as a whole keeps playing.
  const isLive =
    previewStatus === "playing" &&
    previewTransformId === compositeTransformId &&
    (previewGraph?.executionOrder.some((n) => n.nodeId === nodeId) ?? false);

  const inputs = definition?.ports.filter((p) => p.direction === "input") ?? [];
  const outputs = definition?.ports.filter((p) => p.direction === "output") ?? [];
  const sortedParams = [...(definition?.params ?? [])].sort((a, b) => a.param_order - b.param_order);
  const liveValues = nodeParamValues.get(nodeId);

  const tabs: { id: InspectorTab; label: string }[] = [
    { id: "source", label: "Source" },
    { id: "details", label: "Details" },
  ];

  return (
    <div
      className="flex flex-col flex-shrink-0"
      style={{ height: 280, borderTop: "1px solid rgba(255,255,255,0.06)", backgroundColor: "#1a1a1a" }}
    >
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
                  ? { color: "#adc6ff", borderBottom: "2px solid #adc6ff", backgroundColor: "rgba(173,198,255,0.06)" }
                  : { color: "var(--text-muted)", opacity: 0.6 }
              }
            >
              {tab.label}
            </button>
          ))}
          <span className="ml-3 text-[10px] font-mono truncate max-w-[240px]" style={{ color: "var(--text-muted)" }}>
            {definition?.name ?? `#${nodeId}`}
          </span>
        </div>
        <button
          onClick={onClose}
          className="font-mono text-[13px] px-2"
          style={{ color: "var(--text-muted)", background: "none", border: "none", cursor: "pointer" }}
          title="Close"
        >
          ×
        </button>
      </div>

      <div className="flex-1 min-h-0">
        {activeTab === "source" ? (
          <Editor
            height="100%"
            language="rust"
            value={definition?.source_code ?? ""}
            theme="vs-dark"
            options={{
              readOnly: true,
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
        ) : (
          <div className="h-full overflow-y-auto p-4 flex flex-col gap-5">
            <PortsList direction="input" ports={inputs} />
            <PortsList direction="output" ports={outputs} />
            {sortedParams.length > 0 && (
              <section>
                <div
                  className="flex items-center justify-between mb-2 pb-1"
                  style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}
                >
                  <h3 className="text-[9px] font-mono" style={{ color: "var(--text-muted)" }}>
                    PARAMS
                  </h3>
                  {!isLive && (
                    <span className="text-[9px] font-mono" style={{ color: "var(--text-muted)", opacity: 0.6 }}>
                      press Play to edit live
                    </span>
                  )}
                </div>
                <div className="flex flex-col gap-1">
                  {sortedParams.map((param, index) => (
                    <ParamRow
                      key={param.param_id}
                      param={param}
                      liveEnabled={isLive}
                      value={isLive && liveValues?.[index] != null ? liveValues[index] : param.default_value}
                      onChange={(value) => updateNodeParam(nodeId, index, value)}
                    />
                  ))}
                </div>
              </section>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
