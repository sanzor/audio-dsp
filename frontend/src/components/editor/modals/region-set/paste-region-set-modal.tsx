

import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from "../../../ui/dialog";
import { Input } from "../../../ui/input";
import { Button } from "../../../ui/button";
import { useEffect, useState } from "react";
import { Label } from "../../../ui/label";
import type { PasteRegionSetParams } from "@/Stores/PasteParams";
import { useRegionSetStore } from "@/Stores/RegionSetStore";

export interface PasteRegionSetModalProps {
 params:PasteRegionSetParams
  open: boolean;
  onClose: () => void;
  onSubmit: (destTrackId:string,sourceRegionSetId:string, copyRegionSetName: string) => void;
}

export function PasteRegionSetModal({
  params,
  open,
  onClose,
  onSubmit,
}: PasteRegionSetModalProps) {
 const sourceRegionSet = useRegionSetStore(
     (s) => s.regionSets.get(params.source.regionSetId)
   );
 
  const [name, setName] = useState(sourceRegionSet?.name ?? "");
 
 
   useEffect(() => {
     if (open && sourceRegionSet) {
       setName(sourceRegionSet.name);
     }
   }, [open, sourceRegionSet]);
 
   const handleSubmit = () => {
     if (!sourceRegionSet || !name.trim()) return;
 
     onSubmit(params.destination.trackId, sourceRegionSet.id, name.trim());
     onClose();
   };

 return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Paste RegionSet</DialogTitle>
        </DialogHeader>
        <div className="space-y-2">
          <Label>Destination Track</Label>
          <Label>{params.destination.trackId} </Label>
        </div>
        <div className="space-y-2">
          <Label>Source Region Set ID</Label>
          <Label>{params.source.regionSetId} </Label>
        </div>
        <Input value={name} onChange={(e) => setName(e.target.value)} />
        <DialogFooter>
          <Button variant="outline" onClick={onClose}>Cancel</Button>
          <Button onClick={handleSubmit}>Paste</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
