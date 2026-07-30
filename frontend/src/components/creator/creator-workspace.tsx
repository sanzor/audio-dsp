import { CreatorCanvas } from "./canvas";
import { CreatorCodeEditor } from "./code-editor";
import { CompositeCanvas } from "./composite/composite-canvas";
import { useCreatorStore } from "@/Stores/CreatorStore";
import { useGetTransformDefinition } from "@/hooks/transforms/queries";

export function CreatorWorkspace() {
  const selectedId = useCreatorStore((s) => s.selectedTransformId);
  const { data: definition } = useGetTransformDefinition(selectedId);

  if (definition?.kind === "composite") {
    return (
      <div className="flex-1 flex flex-col min-w-0 overflow-hidden">
        <CompositeCanvas />
      </div>
    );
  }

  return (
    <div className="flex-1 flex flex-col min-w-0 overflow-hidden">
      {/* Canvas — top 60% */}
      <div className="flex-[0.6] min-h-0" style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}>
        <CreatorCanvas />
      </div>

      {/* Code editor — bottom 40% */}
      <div className="flex-[0.4] min-h-0 flex flex-col">
        <CreatorCodeEditor />
      </div>
    </div>
  );
}
