import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from "../../../ui/dialog";
import { Label } from "../../../ui/label";
import { Input } from "../../../ui/input";
import { Button } from "../../../ui/button";
import { useTrackStore } from "@/Stores/TrackStore";


export interface DetailsTrackModalProps {
  open: boolean;
  trackId: string;
  onClose: () => void;
}

export function DetailsTrackModal({ trackId, open, onClose }: DetailsTrackModalProps) {
    const track = useTrackStore(state => state.tracks.get(trackId));
      // If regionSet exists, pull the track
  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="space-y-6">
        <DialogHeader>
          <DialogTitle>Track Details</DialogTitle>
        </DialogHeader>

        <div className="space-y-2">
          <Label htmlFor="track-id">Track ID</Label>
          <Input id="track-id" value={track?.trackId} readOnly />
        </div>

        <div className="space-y-2">
          <Label htmlFor="track-name">Name</Label>
          <Input id="track-name" value={track?.trackInfo.name} readOnly />
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={onClose}>
            Exit
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
