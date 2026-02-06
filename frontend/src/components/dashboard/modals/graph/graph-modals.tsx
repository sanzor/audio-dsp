import { useGraphController } from "@/controllers/GraphController";
import { useUIStore } from "@/Stores/UIStore";
import { DetailsGraphModal } from "./details-graph-modal";
import { RenameGraphModal } from "./rename-graph-modal";
import { CreateGraphModal } from "./create-graph-modal";
import { useRegionController } from "@/controllers/RegionController";
import { PasteGraphModal } from "./paste-graph-modal";

export function GraphModals(){
      const modalState = useUIStore(state => state.modalState);
      const closeModal = useUIStore(state => state.closeModal);
      const graphController=useGraphController();
      const regionController=useRegionController();

      if(!modalState)return null;

      switch(modalState.type){
        case "detailsGraph":
            return <DetailsGraphModal 
                     graphId={modalState.graphId}
                     open
                     onClose={closeModal}>
                      
                     </DetailsGraphModal>;
        case "renameGraph":
            return <RenameGraphModal
                graphId={modalState.graphId}
                open
                onClose={closeModal}
                onSubmit={graphController.handleRenameGraph}
            ></RenameGraphModal>;

        case "createGraph":
            return <CreateGraphModal 
                regionId={modalState.regionId}
                onClose={closeModal}
                open
                onSubmit={regionController.handleSubmitCreateGraph}
                >

            </CreateGraphModal>;
        
        case "pasteGraph":
            return <PasteGraphModal
                params={modalState.params}
                onSubmit={(destRegionId,sourceGraphId,copyName)=>
                    regionController.handleSubmitPasteGraph(
                         {source:{graphId:sourceGraphId},destination:{regionId:destRegionId}},copyName)}
                    
            
                onClose={closeModal}
                open
            >
            </PasteGraphModal>
      }
}