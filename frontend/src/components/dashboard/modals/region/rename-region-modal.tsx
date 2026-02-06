import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from "../../../ui/dialog";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { useEffect, useState } from "react";
import { useRegionStore } from "@/Stores/RegionStore";

interface RenameRegionProps {
  open: boolean;
  regionId: string;
  onClose: () => void;
  onSubmit: (regionId: string, newName: string) => void;
}

export function RenameRegionModal({ open, regionId, onClose, onSubmit }: RenameRegionProps) {
  const region = useRegionStore(state => state.regions.get(regionId));
  const [name, setName] = useState(region?.name ?? "");

  useEffect(() => {
    setName(region?.name ?? "");
  }, [region?.name, open]);

  const handleSave = () => {
    if (!name.trim()) return;
    onSubmit(regionId, name.trim());
    onClose();
  };

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Rename Region</DialogTitle>
        </DialogHeader>

        <Input
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder="Enter new name"
        />

        <DialogFooter>
          <Button variant="outline" onClick={onClose}>Cancel</Button>
          <Button onClick={handleSave}>Save</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

