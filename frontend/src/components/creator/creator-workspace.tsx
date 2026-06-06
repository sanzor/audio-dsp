import { CreatorCanvas } from "./creator-canvas";
import { CreatorCodeEditor } from "./creator-code-editor";

export function CreatorWorkspace() {
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
