import WaveSurfer from "wavesurfer.js";
import RegionsPlugin, { type Region } from "wavesurfer.js/dist/plugins/regions.esm.js";
import Minimap from "wavesurfer.js/dist/plugins/minimap.esm.js";
import type { TrackRegionViewModel } from "@/domain/Region/TrackRegionViewModel";

const REGION_GAP = 0.05;

export function createWaveFormPlayer(
  url: string,
  _trackRegions: TrackRegionViewModel[],
  container: HTMLElement,
  onRegionDetails?: (regionId: number) => void,
  onRegionSelect?: (regionId: number) => void,
  onRegionUpdated?: (region: Region, side?: "start" | "end") => Promise<void> | void,
): { wave: WaveSurfer; regions: RegionsPlugin } {
  let activeRegion: Region | null = null;
  const regions = RegionsPlugin.create();

  regions.on("region-in", (region) => {
    activeRegion = region;
  });

  regions.on("region-out", (region) => {
    if (activeRegion === region) {
      activeRegion = null;
    }
  });

  regions.on("region-clicked", (region, event) => {
    event.preventDefault();
    event.stopImmediatePropagation();
    activeRegion = region;
    region.play(true);
    onRegionSelect?.(Number(region.id));
  });

  regions.on("region-double-clicked", (region, event) => {
    event.preventDefault();
    event.stopImmediatePropagation();
    onRegionDetails?.(Number(region.id));
  });

  regions.on("region-updated", (region, side) => {
    void onRegionUpdated?.(region, side);
  });

  const wave = WaveSurfer.create({
    container,
    waveColor: "rgb(100, 152, 200)",
    progressColor: "rgb(100,100,100)",
    url,
    plugins: [
      regions,
      Minimap.create({
        height: 20,
        waveColor: "#ddd",
        progressColor: "#999",
      }),
    ],
    mediaControls: false,
  });

  wave.on("interaction", () => {
    activeRegion = null;
  });

  return { wave, regions };
}

export function clientXToTime(
  clientX: number,
  element: HTMLElement,
  totalDuration: number,
): number {
  const { left, width } = element.getBoundingClientRect();
  if (width <= 0 || totalDuration <= 0) return 0;

  const ratio = Math.min(1, Math.max(0, (clientX - left) / width));
  return ratio * totalDuration;
}

export function isPointInsideRegion(
  time: number,
  regions: TrackRegionViewModel[],
): boolean {
  return regions.some((region) => time >= region.start && time <= region.end);
}

export function clampUpdatedRegionBounds(
  regionId: string,
  start: number,
  end: number,
  regions: TrackRegionViewModel[],
  totalDuration: number,
  side?: "start" | "end",
): { start: number; end: number } {
  const orderedRegions = [...regions].sort((left, right) => left.start - right.start);
  const regionIndex = orderedRegions.findIndex((region) => String(region.regionId) === regionId);
  if (regionIndex === -1) {
    return { start, end };
  }

  const previousRegion = regionIndex > 0 ? orderedRegions[regionIndex - 1] : null;
  const nextRegion = regionIndex < orderedRegions.length - 1 ? orderedRegions[regionIndex + 1] : null;

  const minStart = previousRegion ? previousRegion.end + REGION_GAP : 0;
  const maxEnd = nextRegion ? nextRegion.start - REGION_GAP : totalDuration;
  const minimumLength = 0.01;

  if (side === "start") {
    const nextStart = Math.min(end - minimumLength, maxEnd - minimumLength);
    return {
      start: Math.max(minStart, Math.min(start, nextStart)),
      end: Math.min(end, maxEnd),
    };
  }

  if (side === "end") {
    const previousEnd = Math.max(start + minimumLength, minStart + minimumLength);
    return {
      start: Math.max(start, minStart),
      end: Math.min(maxEnd, Math.max(end, previousEnd)),
    };
  }

  const width = Math.max(minimumLength, end - start);
  let clampedStart = start;
  let clampedEnd = end;

  if (clampedStart < minStart) {
    clampedStart = minStart;
    clampedEnd = minStart + width;
  }

  if (clampedEnd > maxEnd) {
    clampedEnd = maxEnd;
    clampedStart = maxEnd - width;
  }

  clampedStart = Math.max(minStart, clampedStart);
  clampedEnd = Math.min(maxEnd, clampedEnd);

  if (clampedEnd - clampedStart < minimumLength) {
    clampedEnd = Math.min(maxEnd, clampedStart + minimumLength);
    clampedStart = Math.max(minStart, clampedEnd - minimumLength);
  }

  return { start: clampedStart, end: clampedEnd };
}

export function addRegion(
  regions: RegionsPlugin,
  region: TrackRegionViewModel,
  isSelected = false,
  isEditable = false,
): RegionsPlugin {
  regions.addRegion({
    id: String(region.regionId),
    start: region.start,
    end: region.end,
    drag: false,
    resize: isEditable,
    content: region.name,
    color: colorForRegion(region.regionId, isSelected),
  });

  return regions;
}

export function colorForRegion(regionId: string | number, isSelected = false): string {
  const value = String(regionId);
  let hash = 0;

  for (let index = 0; index < value.length; index += 1) {
    hash = (hash * 31 + value.charCodeAt(index)) >>> 0;
  }

  const hue = hash % 360;
  return isSelected
    ? `hsla(${hue}, 98%, 72%, 0.88)`
    : `hsla(${hue}, 72%, 58%, 0.32)`;
}
