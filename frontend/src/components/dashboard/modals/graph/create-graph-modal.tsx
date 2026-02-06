
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { useState } from "react";
import { useRegionStore } from "@/Stores/RegionStore";
import type { CreateGraphParams } from "@/Dtos/Graphs/CreateGraphParams";

interface CreateGraphModalProps {
  regionId: string;
  open: boolean;
  onClose: () => void;
  onSubmit: (params: CreateGraphParams) => void;
  duration?: number; // track duration
}

export function CreateGraphModal({
  regionId,
  open,
  onClose,
  onSubmit
}: CreateGraphModalProps) {
  const [name, setName] = useState("");


  // 🔥 Resolve RegionSet
  const region = useRegionStore(
    (state) => state.regions.get(regionId)
  );

  // 🔥 Resolve Track (parent)



  const handleSubmit = () => {
    if (!name || !region) return;

    onSubmit({
      name,
      region_id: regionId
    });
  };

  if (!region) {
    return null; // Or show fallback if needed
  }

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>Create Graph</DialogTitle>
        </DialogHeader>

        <div className="space-y-4">

          {/* Region Name */}
          <div className="grid grid-cols-4 items-center gap-4">
            <label className="text-right text-sm font-medium">Name</label>
            <Input
              className="col-span-3"
              placeholder="Enter region name"
              value={name}
              onChange={(e) => setName(e.target.value)}
            />
          </div>

        

        
          <div className="text-xs text-muted-foreground">
            Region : {region.name}
          </div>
        </div>

        <DialogFooter>
          <Button variant="ghost" onClick={onClose}>
            Cancel
          </Button>
          <Button disabled={!name} onClick={handleSubmit}>
            Save
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
