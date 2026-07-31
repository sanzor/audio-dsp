import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from "../../../ui/dialog";
import { Button } from "@/components/ui/button";

interface ClearNodesModalProps {
  open: boolean;
  onClose: () => void;
  onConfirm: () => void;
}

export function ClearNodesModal({ open, onClose, onConfirm }: ClearNodesModalProps) {
  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Clear Nodes</DialogTitle>
        </DialogHeader>

        <p style={{ fontSize: "0.9rem", color: "var(--text-muted)" }}>
          Remove all transform nodes from the graph? Input and Output nodes will be kept.
          This cannot be undone.
        </p>

        <DialogFooter>
          <Button variant="outline" onClick={onClose}>Cancel</Button>
          <Button
            onClick={onConfirm}
            style={{ background: "#dc2626", color: "#fff", border: "none" }}
          >
            Yes, Clear
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
