import { Slider } from "@/components/ui/slider";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
  DialogDescription,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { useEffect, useState } from "react";

import { useRegionSetStore } from "@/Stores/RegionSetStore";
import { useTrackStore } from "@/Stores/TrackStore";

import type { CreateRegionParams } from "@/Dtos/Regions/CreateRegionParams";

interface CreateRegionModalProps {
  regionSetId: string;
  open: boolean;
  startTime: number | null;
  endTime: number | null;
  onClose: () => void;
  onSubmit: (params: CreateRegionParams) => void;
  duration?: number; // track duration
}

export function CreateRegionModal({
  regionSetId,
  open,
  startTime,
  endTime,
  onClose,
  onSubmit,
  duration = 100,
}: CreateRegionModalProps) {
  const [name, setName] = useState("");

  const [range, setRange] = useState<[number, number]>([
    startTime ?? 0,
    endTime ?? 5,
  ]);

  // 🔥 Resolve RegionSet
  const regionSet = useRegionSetStore(
    (state) => state.regionSets.get(regionSetId)
  );

  // 🔥 Resolve Track (parent)
  const track = useTrackStore((state) =>
    regionSet ? state.tracks.get(regionSet.trackId) : null
  );

  // Keep slider synced when modal reopens or start/end change
  useEffect(() => {
    setRange([startTime ?? 0, endTime ?? 5]);
  }, [startTime, endTime, open]);

  const handleSubmit = () => {
    const [start, end] = range;
    if (!name || start >= end || !regionSet) return;

    onSubmit({
      name,
      region_set_id: regionSetId,
      start_time: start,
      end_time: end,
    });
  };

  if (!regionSet) {
    return null; // Or show fallback if needed
  }

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>Create Region</DialogTitle>
          <DialogDescription>Select region boundaries</DialogDescription>
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

          {/* Time slider */}
          <div className="space-y-2">
            <span className="text-sm text-muted-foreground">Time Selection</span>
            <Slider
              min={0}
              max={duration}
              step={0.01}
              value={range}
              onValueChange={(val) => setRange(val as [number, number])}
            />
            <div className="flex justify-between text-xs text-muted-foreground">
              <span>{range[0].toFixed(2)}s</span>
              <span>{range[1].toFixed(2)}s</span>
            </div>
          </div>

          {/* Optional: display parent hierarchy */}
          {track && (
            <div className="text-xs text-muted-foreground">
              Track: {track.trackInfo.name}
            </div>
          )}
          <div className="text-xs text-muted-foreground">
            Region Set: {regionSet.name}
          </div>
        </div>

        <DialogFooter>
          <Button variant="ghost" onClick={onClose}>
            Cancel
          </Button>
          <Button disabled={!name || range[0] >= range[1]} onClick={handleSubmit}>
            Save
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
