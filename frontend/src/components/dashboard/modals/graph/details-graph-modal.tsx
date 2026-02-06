import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from "../../../ui/dialog";
import { Label } from "../../../ui/label";
import { Input } from "../../../ui/input";
import { Button } from "../../../ui/button";

import { useRegionStore } from "@/Stores/RegionStore";
import { useRegionSetStore } from "@/Stores/RegionSetStore";
import { useTrackStore } from "@/Stores/TrackStore";
import { useGraphStore } from "@/Stores/GraphStore";

export interface DetailsGraphProps {
  open: boolean;
  graphId: string;
  onClose: () => void;
}

export function DetailsGraphModal({ graphId, open, onClose }: DetailsGraphProps) {
  // Pull the region
  const graph = useGraphStore(state => state.graphs.get(graphId));

  // If region exists, pull the regionSet
  const region = useRegionStore(state => 
    graph ? state.regions.get(graph.regionId) : null
  );

  const regionSet = useRegionSetStore(state => 
    region ? state.regionSets.get(region?.regionSetId) : null
  );

  // If regionSet exists, pull the track
  const track = useTrackStore(state => 
    regionSet ? state.tracks.get(regionSet.trackId) : null
  );

  if (!graph) {
    return null; // Or show a fallback error UI
  }

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="space-y-6">
        <DialogHeader>
          <DialogTitle>Region Details</DialogTitle>
        </DialogHeader>
         <div className="space-y-2">
          <Label>Graph ID</Label>
          <Input value={graph.id} readOnly />
        </div>
        <div className="space-y-2">
          <Label>Region ID</Label>
          <Input value={graph.regionId} readOnly />
        </div>

        <div className="space-y-2">
          <Label>Graph Name</Label>
          <Input value={graph.name} readOnly />
        </div>

        <div className="space-y-2">
          <Label>Region ID</Label>
          <Input value={region?.regionId} readOnly />
        </div>

        {regionSet && (
          <div className="space-y-2">
            <Label>Region Name</Label>
            <Input value={region?.name} readOnly />
          </div>
        )}

        {regionSet && (
          <div className="space-y-2">
            <Label>Region Set Id</Label>
            <Input value={regionSet.id} readOnly />
          </div>
        )}

         {regionSet && (
          <div className="space-y-2">
            <Label>Region Set Name</Label>
            <Input value={regionSet.name} readOnly />
          </div>
        )}
         {track && (
          <div className="space-y-2">
            <Label>Track Id</Label>
            <Input value={track.trackId} readOnly />
          </div>
        )}
        {track && (
          <div className="space-y-2">
            <Label>Track Name</Label>
            <Input value={track.trackInfo.name} readOnly />
          </div>
        )}

        <DialogFooter>
          <Button variant="outline" onClick={onClose}>
            Exit
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
