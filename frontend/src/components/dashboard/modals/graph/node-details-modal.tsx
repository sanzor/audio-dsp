import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { useTransformStore } from "@/Stores/TransformStore";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useEffect, useState } from "react";
import { useGetTransformDefinition } from "@/hooks/transforms/queries";

interface NodeDetailsModalProps {
  nodeId: number | null;
  position?: { x: number; y: number } | null;
  initialParams: Record<string, number>;
  transformId: number | null;
  open: boolean;
  onClose: () => void;
  onSubmitParams: (nodeId: number, params: Record<string, number>) => void;
}

export function NodeDetailsModal({ nodeId, position, initialParams, transformId, open, onClose, onSubmitParams }: NodeDetailsModalProps) {
  const summary = useTransformStore((s) =>
    transformId != null ? s.summaries.get(transformId) : undefined
  );
  const transform = useTransformStore((s) =>
    transformId != null ? s.definitions.get(transformId) : undefined
  );
  useGetTransformDefinition(transformId, open);

  const inputs = transform?.ports.filter((p) => p.direction === "input") ?? [];
  const outputs = transform?.ports.filter((p) => p.direction === "output") ?? [];
  const [paramEntries, setParamEntries] = useState<Array<{ key: string; value: string }>>([]);

  useEffect(() => {
    const source = Object.keys(initialParams).length > 0
      ? initialParams
      : Object.fromEntries(
          [...(transform?.params ?? [])]
            .sort((a, b) => a.param_order - b.param_order)
            .map((param) => [param.name, param.default_value]),
        );
    const entries = Object.entries(source).map(([key, value]) => ({
      key,
      value: String(value),
    }));
    setParamEntries(entries.length > 0 ? entries : [{ key: "", value: "" }]);
  }, [initialParams, open, nodeId, transform]);

  const handleParamChange = (index: number, field: "key" | "value", value: string) => {
    setParamEntries((current) =>
      current.map((entry, entryIndex) =>
        entryIndex === index ? { ...entry, [field]: value } : entry
      )
    );
  };


  const handleApply = () => {
    if (nodeId == null) return;

    const nextParams: Record<string, number> = {};
    for (const entry of paramEntries) {
      const key = entry.key.trim();
      const value = entry.value.trim();
      if (!key || !value) continue;
      const numeric = Number(value);
      if (Number.isNaN(numeric)) continue;
      nextParams[key] = numeric;
    }

    onSubmitParams(nodeId, nextParams);
    onClose();
  };

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{transform?.name ?? summary?.name ?? `Node #${nodeId}`}</DialogTitle>
        </DialogHeader>

        <div className="space-y-4 text-sm">
          {transform?.description && (
            <p className="text-muted-foreground">{transform.description}</p>
          )}

          {nodeId != null && position && (
            <div className="space-y-1">
              <div className="flex justify-between">
                <span className="text-muted-foreground">Node ID</span>
                <span>{nodeId}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Position</span>
                <span>x: {Math.round(position.x)}, y: {Math.round(position.y)}</span>
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

          <div className="space-y-3">

            <div className="space-y-2">
              {paramEntries.map((entry, index) => (
                <div key={`${nodeId ?? "node"}-param-${index}`} className="grid grid-cols-[1fr_1fr_auto] gap-2">
                  <div className="space-y-1">
                    <Label htmlFor={`param-key-${index}`}>Name</Label>
                    <Label htmlFor={`param-key-${index}`}>{entry.key}</Label>
                    {/* <Label
                      id={`param-key-${index}`}
                      value={entry.key}
                      placeholder="gain"
                    /> */}
                  </div>
                  <div className="space-y-1">
                    <Label htmlFor={`param-value-${index}`}>Value</Label>
                    <Input
                      id={`param-value-${index}`}
                      value={entry.value}
                      onChange={(event) => handleParamChange(index, "value", event.target.value)}
                      placeholder="1.25"
                      inputMode="decimal"
                    />
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>

        <DialogFooter>
          <Button onClick={handleApply}>Apply</Button>
          <Button variant="outline" onClick={onClose}>Close</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
