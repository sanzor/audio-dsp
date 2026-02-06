import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from "../../../ui/dialog";
import { Label } from "../../../ui/label";
import { Input } from "../../../ui/input";
import { Button } from "../../../ui/button";

import { useRegionStore } from "@/Stores/RegionStore";
import { useRegionSetStore } from "@/Stores/RegionSetStore";
import { useTrackStore } from "@/Stores/TrackStore";

export interface DetailsRegionProps {
  open: boolean;
  regionId: string;
  onClose: () => void;
}

export function DetailsRegionModal({ regionId, open, onClose }: DetailsRegionProps) {
  // Pull the region
  const region = useRegionStore(state => state.regions.get(regionId));

  // If region exists, pull the regionSet
  const regionSet = useRegionSetStore(state => 
    region ? state.regionSets.get(region.regionSetId) : null
  );

  // If regionSet exists, pull the track
  const track = useTrackStore(state => 
    regionSet ? state.tracks.get(regionSet.trackId) : null
  );

  if (!region) {
    return null; // Or show a fallback error UI
  }

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="space-y-6">
        <DialogHeader>
          <DialogTitle>Region Details</DialogTitle>
        </DialogHeader>

        <div className="space-y-2">
          <Label>Region ID</Label>
          <Input value={region.regionId} readOnly />
        </div>

        <div className="space-y-2">
          <Label>Region Name</Label>
          <Input value={region.name} readOnly />
        </div>

        <div className="space-y-2">
          <Label>Region Set ID</Label>
          <Input value={region.regionSetId} readOnly />
        </div>

        {regionSet && (
          <div className="space-y-2">
            <Label>Region Set Name</Label>
            <Input value={regionSet.name} readOnly />
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
