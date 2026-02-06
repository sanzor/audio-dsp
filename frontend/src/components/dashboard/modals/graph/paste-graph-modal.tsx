

import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from "../../../ui/dialog";
import { Input } from "../../../ui/input";
import { Button } from "../../../ui/button";
import { useEffect, useState } from "react";
import type { PasteGraphParams } from "@/Stores/PasteParams";
import { Label } from "@/components/ui/label";
import { useGraphStore } from "@/Stores/GraphStore";

export interface PasteGraphModalProps {
  params:PasteGraphParams
  open: boolean;
  onClose: () => void;
  onSubmit: (destRegionId:string,sourceGraphId:string, copyGraphName: string) => void;
}

export function PasteGraphModal({
  params,
  open,
  onClose,
  onSubmit,
}: PasteGraphModalProps) {
  const sourceGraph = useGraphStore(
    (s) => s.graphs.get(params.source.graphId)
  );

 const [name, setName] = useState(sourceGraph?.name ?? "");


  useEffect(() => {
    if (open && sourceGraph) {
      setName(sourceGraph.name);
    }
  }, [open, sourceGraph]);

  const handleSubmit = () => {
    if (!sourceGraph || !name.trim()) return;

    onSubmit(params.destination.regionId, sourceGraph.id, name.trim());
    onClose();
  };

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Paste Graph</DialogTitle>
        </DialogHeader>
        <div className="space-y-2">
          <Label>Destination Region</Label>
          <Label>{params.destination.regionId} </Label>
        </div>
        <div className="space-y-2">
          <Label>Source Graph ID</Label>
          <Label>{params.source.graphId} </Label>
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