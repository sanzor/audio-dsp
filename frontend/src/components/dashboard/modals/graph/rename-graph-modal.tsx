import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from "../../../ui/dialog";
import { Input } from "../../../ui/input";
import { Button } from "../../../ui/button";
import { useEffect, useState } from "react";
import { useGraphStore } from "@/Stores/GraphStore";

export interface RenameGraphProps {
  graphId: string; // 👈 allow null
  open: boolean;
  onClose: () => void;
  onSubmit: (regionId: string, newName: string) => void;
}

export function RenameGraphModal({
  graphId,
  open,
  onClose,
  onSubmit,
}: RenameGraphProps) {
  const graph = useGraphStore(state => state.graphs.get(graphId));
  
  const [graphName, setGraphName] = useState(graph?.name ?? "");

  useEffect(() => {
    setGraphName(graph?.name ?? "");
  }, [graph?.name, open]);

  const handleSubmit = () => {
    if (graphName && graphName.trim()) {
      onSubmit(graphId, graphName.trim());
      onClose();
    }
  };

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Rename Graph</DialogTitle>
        </DialogHeader>
        <Input
          value={graphName}
          onChange={(e) => setGraphName(e.target.value)}
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
