

import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from "../../../ui/dialog";
import { Input } from "../../../ui/input";
import { Button } from "../../../ui/button";
import { useEffect, useState } from "react";
import { useTrackStore } from "@/Stores/TrackStore";

export interface PasteTrackModalProps {
  trackId: string; // 👈 allow null
  open: boolean;
  onClose: () => void;
  onSubmit: (trackId: string, copyTrackName: string) => void;
}

export function PasteTrackModal({
  trackId,
  open,
  onClose,
  onSubmit,
}: PasteTrackModalProps) {
   const track = useTrackStore(
       (s) => s.tracks.get(trackId)
     );
   
  const [copyTrackName, setCopyTrackName] = useState(track?.trackInfo.name ?? "");

  useEffect(() => {
    setCopyTrackName(track?.trackInfo.name ?? "");
    }, [track?.trackInfo.name, open]);

  const handleSubmit = () => {
    if (copyTrackName?.trim() && track?.trackId) {
    onSubmit(track.trackId, copyTrackName.trim());
    onClose();
    }
};

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Track to copy</DialogTitle>
        </DialogHeader>
        <Input
          value={copyTrackName}
          onChange={(e) => setCopyTrackName(e.target.value)}
          placeholder="Enter new name"
        />
        <DialogFooter className="mt-4">
          <Button variant="outline" onClick={onClose}>
            Cancel
          </Button>
          <Button onClick={handleSubmit}>Save</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
