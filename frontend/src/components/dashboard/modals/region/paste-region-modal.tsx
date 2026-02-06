

import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from "../../../ui/dialog";
import { Input } from "../../../ui/input";
import { Button } from "../../../ui/button";
import { useEffect, useState } from "react";
import type { PasteRegionParams } from "@/Stores/PasteParams";
import { useRegionStore } from "@/Stores/RegionStore";
import { Label } from "@/components/ui/label";

export interface PasteRegionModalProps {
  params:PasteRegionParams
  open: boolean;
  onClose: () => void;
  onSubmit: (destRegionSetId:string,sourceRegionId:string, copyRegionName: string) => void;
}

export function PasteRegionModal({
  params,
  open,
  onClose,
  onSubmit,
}: PasteRegionModalProps) {
  const sourceRegion = useRegionStore(
    (s) => s.regions.get(params.source.regionId)
  );

 const [name, setName] = useState(sourceRegion?.name ?? "");


  useEffect(() => {
    if (open && sourceRegion) {
      setName(sourceRegion.name);
    }
  }, [open, sourceRegion]);

  const handleSubmit = () => {
    if (!sourceRegion || !name.trim()) return;

    onSubmit(params.destination.regionSetId, sourceRegion.regionId, name.trim());
    onClose();
  };

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Paste Region</DialogTitle>
        </DialogHeader>
        <div className="space-y-2">
          <Label>Destination Region Set</Label>
          <Label>{params.destination.regionSetId} </Label>
        </div>
        <div className="space-y-2">
          <Label>Source Region ID</Label>
          <Label>{params.source.regionId} </Label>
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