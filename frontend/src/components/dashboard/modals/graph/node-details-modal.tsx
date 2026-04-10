import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { useGraphStore } from "@/Stores/GraphStore";
import { useTransformStore } from "@/Stores/TransformStore";

interface NodeDetailsModalProps {
  nodeId: number | null;
  transformId: number | null;
  open: boolean;
  onClose: () => void;
}

export function NodeDetailsModal({ nodeId, transformId, open, onClose }: NodeDetailsModalProps) {
  const node = useGraphStore((s) => {
    if (nodeId == null) return undefined;
    for (const graph of s.graphs.values()) {
      const found = graph.nodes.find((n) => n.id === nodeId);
      if (found) return found;
    }
    return undefined;
  });

  const transform = useTransformStore((s) =>
    transformId != null ? s.transforms.get(transformId) : undefined
  );

  const inputs = transform?.ports.filter((p) => p.direction === "input") ?? [];
  const outputs = transform?.ports.filter((p) => p.direction === "output") ?? [];

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{transform?.name ?? `Node #${nodeId}`}</DialogTitle>
        </DialogHeader>

        <div className="space-y-4 text-sm">
          {transform?.description && (
            <p className="text-muted-foreground">{transform.description}</p>
          )}

          {node && (
            <div className="space-y-1">
              <div className="flex justify-between">
                <span className="text-muted-foreground">Node ID</span>
                <span>{node.id}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Position</span>
                <span>x: {Math.round(node.position.x)}, y: {Math.round(node.position.y)}</span>
              </div>
            </div>
          )}

          {(inputs.length > 0 || outputs.length > 0) && (
            <div className="flex gap-6">
              <div className="flex-1 space-y-1">
                <div className="font-medium">Inputs ({inputs.length})</div>
                <ul className="space-y-1">
                  {inputs.map((p) => (
                    <li key={p.port_id} className="rounded bg-muted px-2 py-1">
                      {p.name}
                      {p.description && (
                        <span className="ml-2 text-xs text-muted-foreground">{p.description}</span>
                      )}
                    </li>
                  ))}
                </ul>
              </div>
              <div className="flex-1 space-y-1">
                <div className="font-medium">Outputs ({outputs.length})</div>
                <ul className="space-y-1">
                  {outputs.map((p) => (
                    <li key={p.port_id} className="rounded bg-muted px-2 py-1">
                      {p.name}
                      {p.description && (
                        <span className="ml-2 text-xs text-muted-foreground">{p.description}</span>
                      )}
                    </li>
                  ))}
                </ul>
              </div>
            </div>
          )}
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={onClose}>Close</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
