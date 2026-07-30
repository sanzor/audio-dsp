import { useCreatorStore } from "@/Stores/CreatorStore";
import { useCreatorPreviewStore } from "@/Stores/CreatorPreviewStore";
import type { TransformPort } from "@/domain/Transform/TransformPort";
import type { TransformParam } from "@/domain/Transform/TransformParam";
import { useGetTransformDefinition } from "@/hooks/transforms/queries";

// Prop-driven (no store reads of its own) so it can be reused by any
// consumer with its own notion of "current value" / "is this live" / "what
// happens on edit" — the right-sidebar TransformPropertiesPanel below (bound
// to useCreatorStore.selectedTransformId's single-transform preview) and the
// composite canvas's per-node bottom-panel Details tab (bound to a specific
// node_id within a composite preview graph — see composite-canvas.tsx and
// agents/decisions/0005-composite-node-inspector.md) both wrap it.
export interface ParamRowProps {
  param: TransformParam;
  value: number;
  liveEnabled: boolean;
  onChange: (value: number) => void;
}

// Uses only min_value/max_value/default_value, which already exist on
// TransformParam; a future unit/step/curve enrichment would slot into this
// same row (e.g. step={param.step ?? 1}, a unit-suffix label) without
// restructuring anything else here.
export function ParamRow({ param, value, liveEnabled, onChange }: ParamRowProps) {
  const min = param.min_value ?? 0;
  const max = param.max_value ?? Math.max(param.default_value * 2, 1);

  return (
    <div
      className="flex flex-col gap-1 px-2 py-1.5 rounded text-xs"
      style={{ backgroundColor: "var(--bg-dark)", border: "1px solid rgba(255,255,255,0.05)" }}
    >
      <div className="flex items-center justify-between">
        <span style={{ color: "var(--text-muted)" }}>{param.name}</span>
        <span className="font-mono" style={{ color: "var(--text-main)" }}>
          {value.toFixed(3)}
        </span>
      </div>
      <input
        type="range"
        min={min}
        max={max}
        step={(max - min) / 200 || 0.01}
        value={value}
        disabled={!liveEnabled}
        onChange={(e) => onChange(Number(e.target.value))}
        title={liveEnabled ? undefined : "Press Play to enable live parameter control"}
        className="w-full"
        style={{ opacity: liveEnabled ? 1 : 0.5 }}
      />
    </div>
  );
}


export interface PortsListProps {
  direction: "input" | "output";
  ports: TransformPort[];
}

export function PortsList({ direction, ports }: PortsListProps) {
  const label = direction === "input" ? "INPUTS" : "OUTPUTS";
  const color = direction === "input" ? "#adc6ff" : "#4ae176";

  return (
    <section>
      <div className="flex items-center justify-between mb-2">
        <h3 className="text-[9px] font-mono" style={{ color: "var(--text-muted)" }}>
          {label}
        </h3>
        <span className="text-[9px] font-mono" style={{ color: "var(--text-muted)", opacity: 0.6 }}>
          from source
        </span>
      </div>

      <div className="flex flex-col gap-1">
        {ports.length === 0 && (
          <span className="text-[10px]" style={{ color: "var(--text-muted)", opacity: 0.5 }}>
            No {direction}s
          </span>
        )}
        {ports.map((port) => (
          <div
            key={port.port_id}
            className="flex items-center gap-2 px-2 py-1 rounded"
            style={{ backgroundColor: "var(--bg-dark)", border: "1px solid rgba(255,255,255,0.05)" }}
          >
            <div className="w-1.5 h-1.5 rounded-full flex-shrink-0" style={{ backgroundColor: color }} />
            <span className="flex-1 text-xs" style={{ color: "var(--text-main)", minWidth: 0 }}>
              {port.name}
            </span>
            {port.kind === "sidechain" && (
              <span
                className="text-[8px] font-mono px-1 rounded"
                style={{ color: "#ffb786", border: "1px solid rgba(255,183,134,0.4)" }}
                title="Sidechain — control/detector signal, silent when unwired"
              >
                sidechain
              </span>
            )}
            {port.cardinality === "many" && (
              <span
                className="text-[8px] font-mono px-1 rounded"
                style={{ color: "#adc6ff", border: "1px solid rgba(173,198,255,0.4)" }}
                title="Many — multiple edges sum into this port"
              >
                many
              </span>
            )}
            <span className="text-[9px] font-mono" style={{ color: "var(--text-muted)" }}>
              {port.direction}
            </span>
          </div>
        ))}
      </div>
    </section>
  );
}

export function TransformPropertiesPanel() {
  const selectedId = useCreatorStore((s) => s.selectedTransformId);
  const { data: definition } = useGetTransformDefinition(selectedId);
  const previewStatus = useCreatorPreviewStore((s) => s.status);
  const previewTransformId = useCreatorPreviewStore((s) => s.previewTransformId);
  const paramValues = useCreatorPreviewStore((s) => s.paramValues);
  const updateParam = useCreatorPreviewStore((s) => s.updateParam);

  if (selectedId == null) {
    return (
      <aside
        className="flex flex-col w-80 flex-shrink-0 items-center justify-center"
        style={{ backgroundColor: "var(--bg-darker)", borderLeft: "1px solid rgba(255,255,255,0.06)" }}
      >
        <span className="text-xs" style={{ color: "var(--text-muted)" }}>
          No transform selected
        </span>
      </aside>
    );
  }

  const inputs = definition?.ports.filter((port) => port.direction === "input") ?? [];
  const outputs = definition?.ports.filter((port) => port.direction === "output") ?? [];

  // Sorted by param_order — this index must match the wasm side's
  // positional param index (see CreatorPreviewStore.paramValues), which is
  // keyed by declaration order, not param_id.
  const sortedParams = [...(definition?.params ?? [])].sort((a, b) => a.param_order - b.param_order);
  const liveEnabled = previewStatus === "playing" && previewTransformId === selectedId;

  return (
    <aside
      className="flex flex-col w-80 flex-shrink-0 overflow-hidden"
      style={{
        backgroundColor: "var(--bg-darker)",
        borderLeft: "1px solid rgba(255,255,255,0.06)",
      }}
    >
      <div
        className="px-4 py-3"
        style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}
      >
        <span className="text-[10px] font-mono font-bold" style={{ color: "var(--text-main)" }}>
          PROPERTIES
        </span>
        <p className="mt-1 text-[10px]" style={{ color: "var(--text-muted)", opacity: 0.7 }}>
          Name, ports & metadata are generated from transform source. Params are live-editable while previewing.
        </p>
      </div>

      <div className="flex-1 overflow-y-auto p-4 flex flex-col gap-5">
        <section className="flex flex-col gap-3">
          <h3 className="text-[9px] font-mono pb-1" style={{ color: "var(--text-muted)", borderBottom: "1px solid rgba(255,255,255,0.06)" }}>
            METADATA
          </h3>
          <div className="flex flex-col gap-1">
            <label className="text-[10px]" style={{ color: "var(--text-muted)" }}>
              Name
            </label>
            <div
              className="w-full rounded px-2 py-1.5 text-xs"
              style={{
                backgroundColor: "var(--bg-dark)",
                border: "1px solid rgba(255,255,255,0.08)",
                color: "var(--text-main)",
              }}
            >
              {definition?.name ?? "Untitled"}
            </div>
          </div>
          <div className="flex flex-col gap-1">
            <label className="text-[10px]" style={{ color: "var(--text-muted)" }}>
              Description
            </label>
            <div
              className="w-full rounded px-2 py-1.5 text-xs min-h-14"
              style={{
                backgroundColor: "var(--bg-dark)",
                border: "1px solid rgba(255,255,255,0.08)",
                color: "var(--text-main)",
              }}
            >
              {definition?.description?.trim() || "No description"}
            </div>
          </div>
        </section>

        <PortsList direction="input" ports={inputs} />
        <PortsList direction="output" ports={outputs} />

        {sortedParams.length > 0 && (
          <section>
            <div className="flex items-center justify-between mb-2 pb-1" style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}>
              <h3 className="text-[9px] font-mono" style={{ color: "var(--text-muted)" }}>
                PARAMS
              </h3>
              {!liveEnabled && (
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
                  liveEnabled={liveEnabled}
                  value={liveEnabled && paramValues[index] != null ? paramValues[index] : param.default_value}
                  onChange={(value) => updateParam(index, value)}
                />
              ))}
            </div>
          </section>
        )}
      </div>
    </aside>
  );
}
