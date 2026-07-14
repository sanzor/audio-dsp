import { useState } from "react";
import { Plus } from "lucide-react";
import { useListTransforms } from "@/hooks/transforms/queries";
import { useCreatorStore } from "@/Stores/CreatorStore";
import { useTransformController } from "@/controllers/TransformController";

export function TransformsSidebar() {
  const [filter, setFilter] = useState("");

  const selectedId = useCreatorStore((s) => s.selectedTransformId);
  const setSelected = useCreatorStore((s) => s.setSelectedTransformId);

  const query = useListTransforms();
  const transformController = useTransformController();

  const allTransforms = query.data?.pages.flatMap((p) => p.transforms) ?? [];
  const filtered = filter
    ? allTransforms.filter((t) => t.name.toLowerCase().includes(filter.toLowerCase()))
    : allTransforms;

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
        className="px-3 py-2.5 flex items-center justify-between"
        style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}
      >
        <span className="text-[10px] font-mono font-bold" style={{ color: "var(--text-main)" }}>
          TRANSFORMS
        </span>
        <button
          onClick={transformController.handleCreateTransform}
          className="w-6 h-6 rounded flex items-center justify-center hover:bg-white/10 transition-colors"
          title="New Transform"
          style={{ background: "none", border: "none", padding: 0, cursor: "pointer" }}
        >
          <Plus className="w-4 h-4" style={{ color: "var(--text-muted)" }} />
        </button>
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
    </aside>
  );
}
