import { useEffect, useState } from "react";
import { useCreatorStore } from "@/Stores/CreatorStore";
import type { TransformPort } from "@/domain/Transform/TransformPort";
import { useGetTransformDefinition } from "@/hooks/transforms/queries";
import { useSaveTransform } from "@/hooks/transforms/mutations";


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
  const saveMutation = useSaveTransform(selectedId ?? -1);

  const [name, setName] = useState("");
  const [description, setDescription] = useState("");

  useEffect(() => {
    setName(definition?.name ?? "");
    setDescription(definition?.description ?? "");
  }, [definition?.name, definition?.description]);

  const isDirty = name !== (definition?.name ?? "") || description !== (definition?.description ?? "");

  function handleSave() {
    if (selectedId == null || !isDirty) return;
    saveMutation.mutate({ name, description: description || undefined });
  }

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
        className="px-4 py-3 flex items-center justify-between"
        style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}
      >
        <div>
          <span className="text-[10px] font-mono font-bold" style={{ color: "var(--text-main)" }}>
            PROPERTIES
          </span>
          <p className="mt-1 text-[10px]" style={{ color: "var(--text-muted)", opacity: 0.7 }}>
            Ports/params are read-only, generated from transform source.
          </p>
        </div>
        <button
          onClick={handleSave}
          disabled={!isDirty || saveMutation.isPending}
          className="font-mono font-bold px-2.5 py-0.5 rounded text-[10px] transition-colors flex-shrink-0"
          style={{
            color: "#adc6ff",
            border: "1px solid rgba(173,198,255,0.4)",
            opacity: !isDirty || saveMutation.isPending ? 0.5 : 1,
          }}
        >
          {saveMutation.isPending ? "Saving…" : "Save"}
        </button>
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
            <input
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full rounded px-2 py-1.5 text-xs"
              style={{
                backgroundColor: "var(--bg-dark)",
                border: "1px solid rgba(255,255,255,0.08)",
                color: "var(--text-main)",
                outline: "none",
              }}
            />
          </div>
          <div className="flex flex-col gap-1">
            <label className="text-[10px]" style={{ color: "var(--text-muted)" }}>
              Description
            </label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              className="w-full rounded px-2 py-1.5 text-xs min-h-14 resize-none"
              style={{
                backgroundColor: "var(--bg-dark)",
                border: "1px solid rgba(255,255,255,0.08)",
                color: "var(--text-main)",
                outline: "none",
              }}
            />
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
