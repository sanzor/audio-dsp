import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from "../../../ui/dialog";
import { Button } from "@/components/ui/button";
import { useRegionStore } from "@/Stores/RegionStore";

interface DeleteRegionModalProps {
  open: boolean;
  regionId: number;
  onClose: () => void;
  onConfirm: (regionId: number) => void;
}

export function DeleteRegionModal({ open, regionId, onClose, onConfirm }: DeleteRegionModalProps) {
  const region = useRegionStore((s) => s.regions.get(regionId));

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Delete Region</DialogTitle>
        </DialogHeader>

        <p style={{ fontSize: "0.9rem", color: "var(--text-muted)" }}>
          Delete <strong style={{ color: "var(--text-main)" }}>{region?.name ?? "this region"}</strong>?
          This will also remove its graph. This cannot be undone.
        </p>

        <DialogFooter>
          <Button variant="outline" onClick={onClose}>Cancel</Button>
          <Button variant="destructive" onClick={() => onConfirm(regionId)}>Delete</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
