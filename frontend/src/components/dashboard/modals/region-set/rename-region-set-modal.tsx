

import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from "../../../ui/dialog";
import { Input } from "../../../ui/input";
import { Button } from "../../../ui/button";
import { useEffect, useState } from "react";
import { useRegionSetStore } from "@/Stores/RegionSetStore";

export interface RenameRegionSetProps {
  regionSetId:string; // 👈 allow null
  open: boolean;
  onClose: () => void;
  onSubmit: (regionSetId: string, newName: string) => void;
}

export function RegionSetRenameModal({
  regionSetId,
  open,
  onClose,
  onSubmit,
}: RenameRegionSetProps) {
  const regionSet = useRegionSetStore(state => state.regionSets.get(regionSetId));
    const [name, setName] = useState(regionSet?.name ?? "");
  
    useEffect(() => {
      setName(regionSet?.name ?? "");
    }, [regionSet?.name, open]);
  
    const handleSave = () => {
      if (!name.trim()) return;
      onSubmit(regionSetId, name.trim());
      onClose();
    };

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Rename Region Set</DialogTitle>
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
