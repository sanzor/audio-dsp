// RegionContextMenuContainer.tsx - Updated
import { useUIStore } from "@/Stores/UIStore";
import { useGraphController } from "@/controllers/GraphController";
import { GraphContextMenu } from "./graph-context-menu";


export function GraphContextMenuContainer() {
  const rightClickContext=useUIStore(state=>state.rightClickContext);
  const closeContextMenu=useUIStore(state=>state.closeContextMenu);
  const controller=useGraphController();

  if (rightClickContext?.type !== "graph") return null;
 //used typed clipboards
  const { graphId, x, y } = rightClickContext;

  return (
     <GraphContextMenu
       x={x}
       y={y}
       graphId={graphId}
       onClose={closeContextMenu}
       onDetails={()=>controller.handleDetailsGraph(graphId)}
       onRename={()=>controller.handleRenameGraph(graphId)}
       onCopy={()=>controller.handleCopyGraph(graphId)}
       onRemove={() => controller.handleDeleteGraph(graphId)}
     />
   );
 
}
