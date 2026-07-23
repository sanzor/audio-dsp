import { useCreatorStore } from "@/Stores/CreatorStore";
import type { TransformPort } from "@/domain/Transform/TransformPort";
import { useGetTransformDefinition } from "@/hooks/transforms/queries";


interface PortsListProps {
  direction: "input" | "output";
  ports: TransformPort[];
}

function PortsList({ direction, ports }: PortsListProps) {
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
          Read-only. Generated from transform source.
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

        {definition?.params && definition.params.length > 0 && (
          <section>
            <h3 className="text-[9px] font-mono mb-2 pb-1" style={{ color: "var(--text-muted)", borderBottom: "1px solid rgba(255,255,255,0.06)" }}>
              PARAMS
            </h3>
            <div className="flex flex-col gap-1">
              {definition.params.map((param) => (
                <div
                  key={param.param_id}
                  className="flex items-center justify-between px-2 py-1 rounded text-xs"
                  style={{ backgroundColor: "var(--bg-dark)", border: "1px solid rgba(255,255,255,0.05)" }}
                >
                  <span style={{ color: "var(--text-muted)" }}>{param.name}</span>
                  <span className="font-mono" style={{ color: "var(--text-main)" }}>
                    {param.default_value}
                  </span>
                </div>
              ))}
            </div>
          </section>
        )}
      </div>
    </aside>
  );
}
