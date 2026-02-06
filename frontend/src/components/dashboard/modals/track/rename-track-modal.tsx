


import { useEffect, useState } from "react";
import { Button } from "../../../ui/button";
import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from "../../../ui/dialog";
import { Input } from "../../../ui/input";
import { useTrackStore } from "@/Stores/TrackStore";


export interface RenameTrackModalProps {
  trackId:string; // 👈 allow null
  open: boolean;
  onClose: () => void;
  onSubmit: (trackId: string, newName: string) => void;
}

export function RenameTrackModal({
  trackId,
  open,
  onClose,
  onSubmit,
}: RenameTrackModalProps) {
   const track = useTrackStore(state => state.tracks.get(trackId));
   const [name, setName] = useState(track?.trackInfo.name ?? "");
  
    useEffect(() => {
      setName(track?.trackInfo.name ?? "");
    }, [track?.trackInfo.name, open]);
  
    const handleSave = () => {
      if (!name.trim()) return;
      onSubmit(trackId, name.trim());
      onClose();
    };

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Rename Track</DialogTitle>
        </DialogHeader>
        <Input
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder="Enter new name"
        />
        <DialogFooter className="mt-4">
          <Button variant="outline" onClick={onClose}>
            Cancel
          </Button>
          <Button onClick={handleSave}>Save</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
