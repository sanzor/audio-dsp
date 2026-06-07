import { useState } from "react";
import { useListTransforms } from "@/hooks/transforms/queries";
import { useCreateTransform } from "@/hooks/transforms/mutations";
import { useCreatorStore } from "@/Stores/CreatorStore";

export function CreatorLeftPanel() {
  const [filter, setFilter] = useState("");
  const [creating, setCreating] = useState(false);
  const [newName, setNewName] = useState("");
  const [newDesc, setNewDesc] = useState("");

  const selectedId = useCreatorStore((s) => s.selectedTransformId);
  const setSelected = useCreatorStore((s) => s.setSelectedTransformId);

  const query = useListTransforms();
  const createMutation = useCreateTransform();

  const allTransforms = query.data?.pages.flatMap((p) => p.transforms) ?? [];
  const filtered = filter
    ? allTransforms.filter((t) => t.name.toLowerCase().includes(filter.toLowerCase()))
    : allTransforms;

  function handleCreate() {
    if (!newName.trim()) return;
    createMutation.mutate(
      { name: newName.trim(), description: newDesc.trim() || undefined },
      {
        onSuccess: (def) => {
          setSelected(def.transform_id);
          setCreating(false);
          setNewName("");
          setNewDesc("");
        },
      }
    );
  }

  return (
    <aside
      className="flex flex-col w-64 flex-shrink-0 overflow-hidden"
      style={{
        backgroundColor: "var(--bg-darker)",
        borderRight: "1px solid rgba(255,255,255,0.06)",
      }}
    >
      {/* Header */}
      <div
        className="px-3 py-2.5"
        style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}
      >
        <span className="text-[10px] font-mono font-bold" style={{ color: "var(--text-main)" }}>
          TRANSFORMS
        </span>
      </div>

      {/* Filter input */}
      <div className="px-3 py-2" style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}>
        <input
          type="text"
          placeholder="Filter..."
          value={filter}
          onChange={(e) => setFilter(e.target.value)}
          className="w-full rounded px-2 py-1 text-xs"
          style={{
            backgroundColor: "var(--bg-dark)",
            border: "1px solid rgba(255,255,255,0.08)",
            color: "var(--text-main)",
            outline: "none",
          }}
        />
      </div>

      {/* List */}
      <div className="flex-1 overflow-y-auto py-1">
        {query.isLoading && (
          <div className="px-4 py-3 text-xs" style={{ color: "var(--text-muted)" }}>
            Loading...
          </div>
        )}
        {filtered.map((t) => (
          <button
            key={t.transform_id}
            onClick={() => setSelected(t.transform_id)}
            className="w-full flex flex-col h-auto py-2 px-4 text-left transition-colors"
            style={
              selectedId === t.transform_id
                ? {
                    color: "#adc6ff",
                    borderLeft: "2px solid #adc6ff",
                    backgroundColor: "rgba(173,198,255,0.08)",
                  }
                : { color: "var(--text-muted)", borderLeft: "2px solid transparent" }
            }
          >
            <span className="text-xs">{t.name}</span>
            {t.description && (
              <span
                className="text-[10px] mt-0.5 truncate w-full"
                style={{ color: "var(--text-muted)", opacity: 0.7 }}
              >
                {t.description}
              </span>
            )}
          </button>
        ))}

        {!query.isLoading && filtered.length === 0 && (
          <div className="px-4 py-3 text-xs" style={{ color: "var(--text-muted)" }}>
            {filter ? "No matches." : "No transforms yet."}
          </div>
        )}

        {query.hasNextPage && (
          <button
            onClick={() => query.fetchNextPage()}
            disabled={query.isFetchingNextPage}
            className="w-full text-[10px] font-mono py-2 transition-colors"
            style={{ color: "var(--text-muted)" }}
          >
            {query.isFetchingNextPage ? "Loading..." : "Load more"}
          </button>
        )}
      </div>

      {/* New transform form / button */}
      <div className="p-2" style={{ borderTop: "1px solid rgba(255,255,255,0.06)" }}>
        {creating ? (
          <div className="flex flex-col gap-2">
            <input
              autoFocus
              type="text"
              placeholder="Transform name"
              value={newName}
              onChange={(e) => setNewName(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === "Enter") handleCreate();
                if (e.key === "Escape") setCreating(false);
              }}
              className="w-full rounded px-2 py-1 text-xs"
              style={{
                backgroundColor: "var(--bg-dark)",
                border: "1px solid #adc6ff",
                color: "var(--text-main)",
                outline: "none",
              }}
            />
            <input
              type="text"
              placeholder="Description (optional)"
              value={newDesc}
              onChange={(e) => setNewDesc(e.target.value)}
              className="w-full rounded px-2 py-1 text-xs"
              style={{
                backgroundColor: "var(--bg-dark)",
                border: "1px solid rgba(255,255,255,0.08)",
                color: "var(--text-main)",
                outline: "none",
              }}
            />
            <div className="flex gap-2">
              <button
                onClick={() => setCreating(false)}
                className="flex-1 h-7 rounded text-[10px] font-mono"
                style={{ border: "1px solid rgba(255,255,255,0.12)", color: "var(--text-muted)" }}
              >
                Cancel
              </button>
              <button
                onClick={handleCreate}
                disabled={!newName.trim() || createMutation.isPending}
                className="flex-1 h-7 rounded text-[10px] font-mono font-bold"
                style={{ backgroundColor: "#adc6ff", color: "#002e6a", opacity: !newName.trim() ? 0.5 : 1 }}
              >
                {createMutation.isPending ? "..." : "Create"}
              </button>
            </div>
          </div>
        ) : (
          <button
            onClick={() => setCreating(true)}
            className="w-full h-8 text-[10px] font-mono rounded"
            style={{ border: "1px solid #adc6ff", color: "#adc6ff" }}
          >
            + New Transform
          </button>
        )}
      </div>
    </aside>
  );
}
