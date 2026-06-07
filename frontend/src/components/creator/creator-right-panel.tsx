import { useEffect, useState } from "react";
import { useCreatorStore } from "@/Stores/CreatorStore";
import { useGetTransformDefinition } from "@/hooks/transforms/queries";
import { useSaveTransform, type LocalPort } from "@/hooks/transforms/mutations";
import type { TransformPort } from "@/domain/Transform/Transform";

// ─── Port list editor ─────────────────────────────────────────────────────────

interface PortsEditorProps {
  direction: "input" | "output";
  ports: LocalPort[];
  onAdd: () => void;
  onRemove: (index: number) => void;
  onRename: (index: number, name: string) => void;
}

function PortsEditor({ direction, ports, onAdd, onRemove, onRename }: PortsEditorProps) {
  const label = direction === "input" ? "INPUTS" : "OUTPUTS";
  const color = direction === "input" ? "#adc6ff" : "#4ae176";

  return (
    <section>
      <div className="flex items-center justify-between mb-2">
        <h3 className="text-[9px] font-mono" style={{ color: "var(--text-muted)" }}>
          {label}
        </h3>
        <button
          onClick={onAdd}
          className="text-[9px] font-mono px-1.5 py-0.5 rounded transition-colors"
          style={{ color, border: `1px solid ${color}`, opacity: 0.8 }}
        >
          + add
        </button>
      </div>

      <div className="flex flex-col gap-1">
        {ports.length === 0 && (
          <span className="text-[10px]" style={{ color: "var(--text-muted)", opacity: 0.5 }}>
            No {direction}s
          </span>
        )}
        {ports.map((port, i) => (
          <div
            key={i}
            className="flex items-center gap-2 px-2 py-1 rounded"
            style={{ backgroundColor: "var(--bg-dark)", border: "1px solid rgba(255,255,255,0.05)" }}
          >
            <div className="w-1.5 h-1.5 rounded-full flex-shrink-0" style={{ backgroundColor: color }} />
            <input
              type="text"
              value={port.name}
              onChange={(e) => onRename(i, e.target.value)}
              className="flex-1 bg-transparent text-xs outline-none"
              style={{ color: "var(--text-main)", minWidth: 0 }}
            />
            {port.port_id == null && (
              <span
                className="text-[8px] font-mono px-1 rounded"
                style={{ color: "#ffb786", backgroundColor: "rgba(255,183,134,0.1)" }}
              >
                new
              </span>
            )}
            <button
              onClick={() => onRemove(i)}
              className="text-[10px] leading-none flex-shrink-0 opacity-40 hover:opacity-100 transition-opacity"
              style={{ color: "var(--text-muted)" }}
            >
              ×
            </button>
          </div>
        ))}
      </div>
    </section>
  );
}

// ─── Main panel ───────────────────────────────────────────────────────────────

export function CreatorRightPanel() {
  const selectedId = useCreatorStore((s) => s.selectedTransformId);
  const { data: definition } = useGetTransformDefinition(selectedId);

  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [ports, setPorts] = useState<LocalPort[]>([]);

  const saveMutation = useSaveTransform(selectedId ?? 0);

  // Sync local state when the fetched definition changes.
  useEffect(() => {
    if (!definition) return;
    setName(definition.name);
    setDescription(definition.description ?? "");
    setPorts(
      definition.ports.map((p) => ({
        port_id: p.port_id,
        name: p.name,
        direction: p.direction,
        port_order: p.port_order,
      }))
    );
  }, [definition]);

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

  const inputs = ports.filter((p) => p.direction === "input");
  const outputs = ports.filter((p) => p.direction === "output");

  function updateDirection(direction: "input" | "output", updated: LocalPort[]) {
    const other = ports.filter((p) => p.direction !== direction);
    setPorts([...other, ...updated]);
  }

  function addPort(direction: "input" | "output") {
    const sameDir = ports.filter((p) => p.direction === direction);
    setPorts((prev) => [
      ...prev,
      { name: `${direction === "input" ? "in" : "out"}_${sameDir.length + 1}`, direction, port_order: sameDir.length },
    ]);
  }

  function removePort(direction: "input" | "output", localIndex: number) {
    const sameDir = ports.filter((p) => p.direction === direction);
    const portToRemove = sameDir[localIndex];
    setPorts((prev) => prev.filter((p) => p !== portToRemove));
  }

  function renamePort(direction: "input" | "output", localIndex: number, newName: string) {
    const sameDir = ports.filter((p) => p.direction === direction);
    const target = sameDir[localIndex];
    setPorts((prev) => prev.map((p) => (p === target ? { ...p, name: newName } : p)));
  }

  function handleSave() {
    if (!definition || !selectedId) return;
    saveMutation.mutate({
      name,
      description: description || undefined,
      ports,
      originalPorts: definition.ports,
    });
  }

  return (
    <aside
      className="flex flex-col w-80 flex-shrink-0 overflow-hidden"
      style={{
        backgroundColor: "var(--bg-darker)",
        borderLeft: "1px solid rgba(255,255,255,0.06)",
      }}
    >
      {/* Header */}
      <div
        className="px-4 py-3"
        style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}
      >
        <span className="text-[10px] font-mono font-bold" style={{ color: "var(--text-main)" }}>
          PROPERTIES
        </span>
      </div>

      {/* Scrollable content */}
      <div className="flex-1 overflow-y-auto p-4 flex flex-col gap-5">

        {/* Metadata */}
        <section className="flex flex-col gap-3">
          <h3 className="text-[9px] font-mono pb-1" style={{ color: "var(--text-muted)", borderBottom: "1px solid rgba(255,255,255,0.06)" }}>
            METADATA
          </h3>
          <div className="flex flex-col gap-1">
            <label className="text-[10px]" style={{ color: "var(--text-muted)" }}>Name</label>
            <input
              type="text"
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
            <label className="text-[10px]" style={{ color: "var(--text-muted)" }}>Description</label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              rows={2}
              className="w-full rounded px-2 py-1.5 text-xs resize-none"
              style={{
                backgroundColor: "var(--bg-dark)",
                border: "1px solid rgba(255,255,255,0.08)",
                color: "var(--text-main)",
                outline: "none",
              }}
            />
          </div>
        </section>

        {/* Inputs */}
        <PortsEditor
          direction="input"
          ports={inputs}
          onAdd={() => addPort("input")}
          onRemove={(i) => removePort("input", i)}
          onRename={(i, n) => renamePort("input", i, n)}
        />

        {/* Outputs */}
        <PortsEditor
          direction="output"
          ports={outputs}
          onAdd={() => addPort("output")}
          onRemove={(i) => removePort("output", i)}
          onRename={(i, n) => renamePort("output", i, n)}
        />

        {/* Params (read-only, no backend CRUD yet) */}
        {definition?.params && definition.params.length > 0 && (
          <section>
            <h3 className="text-[9px] font-mono mb-2 pb-1" style={{ color: "var(--text-muted)", borderBottom: "1px solid rgba(255,255,255,0.06)" }}>
              PARAMS (read-only)
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

      {/* Save */}
      <div className="p-3" style={{ borderTop: "1px solid rgba(255,255,255,0.06)" }}>
        {saveMutation.isError && (
          <p className="text-[10px] mb-2" style={{ color: "#f87171" }}>
            Save failed.
          </p>
        )}
        <button
          onClick={handleSave}
          disabled={saveMutation.isPending}
          className="w-full h-8 rounded text-[10px] font-mono font-bold transition-opacity hover:opacity-90"
          style={{ backgroundColor: "#adc6ff", color: "#002e6a", opacity: saveMutation.isPending ? 0.6 : 1 }}
        >
          {saveMutation.isPending ? "Saving..." : "Save"}
        </button>
      </div>
    </aside>
  );
}
