import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { useTransformStore } from "@/Stores/TransformStore";
import { useGetTransformDefinition } from "@/hooks/transforms/queries";

interface TransformDetailsModalProps {
  transformId: number;
  open: boolean;
  onClose: () => void;
}

export function TransformDetailsModal({ transformId, open, onClose }: TransformDetailsModalProps) {
  const summary = useTransformStore((s) => s.summaries.get(transformId));
  const transform = useTransformStore((s) => s.definitions.get(transformId));
  const transformQuery = useGetTransformDefinition(transformId, open);

  if (!transform && transformQuery.isLoading && summary) {
    return (
      <Dialog open={open} onOpenChange={onClose}>
        <DialogContent className="sm:max-w-md">
          <DialogHeader>
            <DialogTitle>{summary.name}</DialogTitle>
          </DialogHeader>
          <div className="text-sm text-muted-foreground">Loading transform details...</div>
          <DialogFooter>
            <Button variant="outline" onClick={onClose}>Close</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    );
  }

  if (!transform) return null;

  const inputs = transform.ports.filter((p) => p.direction === "input");
  const outputs = transform.ports.filter((p) => p.direction === "output");
  const params = [...transform.params].sort((a, b) => a.param_order - b.param_order);

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

          <div className="space-y-1">
            <div className="font-medium">Parameters ({params.length})</div>
            {params.length === 0 ? (
              <div className="text-muted-foreground text-xs">None</div>
            ) : (
              <ul className="space-y-1">
                {params.map((param) => (
                  <li key={param.param_id} className="rounded bg-muted px-2 py-1">
                    <div className="flex items-center justify-between gap-3">
                      <span>{param.name}</span>
                      <span className="text-xs text-muted-foreground">default {param.default_value}</span>
                    </div>
                    {(param.min_value != null || param.max_value != null || param.description) && (
                      <div className="text-xs text-muted-foreground">
                        {param.description ?? ""}
                        {param.min_value != null || param.max_value != null
                          ? ` ${param.description ? "· " : ""}${param.min_value ?? "-inf"} to ${param.max_value ?? "inf"}`
                          : ""}
                      </div>
                    )}
                  </li>
                ))}
              </ul>
            )}
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={onClose}>Close</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
