import { useUIStore } from "@/Stores/UIStore";
import { useTrackController } from "@/controllers/TrackController";
import { useRegionSetController } from "@/controllers/RegionSetController";
import { useRegionController } from "@/controllers/RegionController";
import { useGraphController } from "@/controllers/GraphController";
import { TrackContextMenu } from "./track-context-menu";
import { RegionSetContextMenu } from "./region-set-context-menu";
import { RegionContextMenu } from "./region-context-menu";
import { GraphContextMenu } from "./graph-context-menu";
import { WaveformRegionContextMenu } from "./waveform-region-context-menu";
import { WaveformTimelineContextMenu } from "./waveform-timeline-context-menu";

export function GlobalContextMenu() {
  const rightClickContext = useUIStore((state) => state.rightClickContext);
  const clipboard = useUIStore((state) => state.clipboard);
  const closeContextMenu = useUIStore((state) => state.closeContextMenu);

  const trackController = useTrackController();
  const regionSetController = useRegionSetController();
  const regionController = useRegionController();
  const graphController = useGraphController();

  if (!rightClickContext) return null;

  switch (rightClickContext.type) {
    case "track": {
      const { trackId, x, y } = rightClickContext;
      return (
        <TrackContextMenu
          x={x}
          y={y}
          trackId={trackId}
          onCreateRegionSet={() => trackController.handleCreateRegionSet(trackId)}
          onDetails={() => trackController.handleDetailsTrack(trackId)}
          onRename={() => trackController.handleRenameTrack(trackId)}
          onCopyTrack={() => trackController.handleCopyTrack(trackId)}
          onPasteRegionSet={() => regionSetController.handlePasteRegionSet(trackId)}
          canPasteRegionSet={clipboard?.type === "regionSet"}
          onRemove={() => trackController.handleDeleteTrack(trackId)}
          onClose={closeContextMenu}
        />
      );
    }

    case "regionSet": {
      const { regionSetId, x, y } = rightClickContext;
      return (
        <RegionSetContextMenu
          x={x}
          y={y}
          regionSetId={regionSetId}
          onCreateRegion={() => regionSetController.handleCreateRegion(regionSetId)}
          onDetails={() => regionSetController.handleDetailsRegionSet(regionSetId)}
          onRename={() => regionSetController.handleEditRegionSet(regionSetId)}
          onCopyRegionSet={() => regionSetController.handleCopyRegionSet(regionSetId)}
          onPasteRegion={() => regionSetController.handlePasteRegion(regionSetId)}
          canPasteRegion={clipboard?.type === "region"}
          onRemove={() => regionSetController.handleDeleteRegionSet(regionSetId)}
          onClose={closeContextMenu}
        />
      );
    }

    case "region": {
      const { regionId, x, y } = rightClickContext;
      return (
        <RegionContextMenu
          x={x}
          y={y}
          regionId={regionId}
          onCreateGraph={() => regionController.handleCreateGraph(regionId)}
          onDetails={() => regionController.handleDetailsRegion(regionId)}
          onRename={() => regionController.handleEditRegion(regionId)}
          onCopyRegion={() => regionController.handleCopyRegion(regionId)}
          onPasteGraph={() => regionController.handlePasteGraph(regionId)}
          canPasteGraph={clipboard?.type === "graph"}
          onRemove={() => regionController.handleDeleteRegion(regionId)}
          onClose={closeContextMenu}
        />
      );
    }

    case "graph": {
      const { graphId, x, y } = rightClickContext;
      return (
        <GraphContextMenu
          x={x}
          y={y}
          graphId={graphId}
          onDetails={() => graphController.handleDetailsGraph(graphId)}
          onRename={() => graphController.handleRenameGraph(graphId)}
          onCopy={() => graphController.handleCopyGraph(graphId)}
          onRemove={() => graphController.handleDeleteGraph(graphId)}
          onClose={closeContextMenu}
        />
      );
    }

    case "waveform_region": {
      const { regionId, x, y } = rightClickContext;
      return (
        <WaveformRegionContextMenu
          x={x}
          y={y}
          regionId={regionId}
          onDetailsRegion={() => regionController.handleDetailsRegion(regionId)}
          onEditRegion={() => regionController.handleEditRegion(regionId)}
          onCopyRegion={() => regionController.handleCopyRegion(regionId)}
          onDeleteRegion={() => regionController.handleDeleteRegion(regionId)}
          onClose={closeContextMenu}
        />
      );
    }

    case "waveform_timeline": {
      const { regionSetId, x, y, time } = rightClickContext;
      return (
        <WaveformTimelineContextMenu
          x={x}
          y={y}
          regionSetId={regionSetId}
          startTime={time}
          onCreateRegionClick={regionSetController.handleCreateRegion}
          onClose={closeContextMenu}
        />
      );
    }

    default:
      return null;
  }
}
