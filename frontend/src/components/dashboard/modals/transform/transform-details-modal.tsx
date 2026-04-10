import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { useTransformStore } from "@/Stores/TransformStore";

interface TransformDetailsModalProps {
  transformId: number;
  open: boolean;
  onClose: () => void;
}

export function TransformDetailsModal({ transformId, open, onClose }: TransformDetailsModalProps) {
  const transform = useTransformStore((s) => s.transforms.get(transformId));

  if (!transform) return null;

  const inputs = transform.ports.filter((p) => p.direction === "input");
  const outputs = transform.ports.filter((p) => p.direction === "output");

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{transform.name}</DialogTitle>
        </DialogHeader>

        <div className="space-y-4 text-sm">
          {transform.description && (
            <p className="text-muted-foreground">{transform.description}</p>
          )}

          <div className="flex gap-6">
            <div className="flex-1 space-y-1">
              <div className="font-medium">Inputs ({inputs.length})</div>
              {inputs.length === 0 ? (
                <div className="text-muted-foreground text-xs">None</div>
              ) : (
                <ul className="space-y-1">
                  {inputs.map((p) => (
                    <li key={p.port_id} className="rounded bg-muted px-2 py-1">
                      <span>{p.name}</span>
                      {p.description && (
                        <span className="ml-2 text-xs text-muted-foreground">{p.description}</span>
                      )}
                    </li>
                  ))}
                </ul>
              )}
            </div>

            <div className="flex-1 space-y-1">
              <div className="font-medium">Outputs ({outputs.length})</div>
              {outputs.length === 0 ? (
                <div className="text-muted-foreground text-xs">None</div>
              ) : (
                <ul className="space-y-1">
                  {outputs.map((p) => (
                    <li key={p.port_id} className="rounded bg-muted px-2 py-1">
                      <span>{p.name}</span>
                      {p.description && (
                        <span className="ml-2 text-xs text-muted-foreground">{p.description}</span>
                      )}
                    </li>
                  ))}
                </ul>
              )}
            </div>
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={onClose}>Close</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
