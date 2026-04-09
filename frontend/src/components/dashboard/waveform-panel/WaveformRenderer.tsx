import { useEffect, useRef, useState, type MutableRefObject } from "react";
import type WaveSurfer from "wavesurfer.js";
import type RegionsPlugin from "wavesurfer.js/dist/plugins/regions.esm.js";
import type { TrackRegionSetViewModel } from "@/domain/RegionSet/TrackRegionSetViewModel";
import type { EditingRegionBounds } from "@/Stores/UIStore";
import { CreateRegionOverlay } from "./CreateRegionOverlay";
import type { WaveSurferPlaybackController } from "./useWaveSurferPlaybackController";
import {
    addRegion,
    clampUpdatedRegionBounds,
    clientXToTime,
    colorForRegion,
    createWaveFormPlayer,
    isPointInsideRegion,
} from "./waveformEngine";

export interface WaveformRendererProps{
    regionSet?: TrackRegionSetViewModel,
    url:string|null,
    waveRef: MutableRefObject<WaveSurfer | null>,
    playback: WaveSurferPlaybackController,
    onRegionDetails?:(regionId:number)=>void,
    onDeleteRegion?:(regionId:number)=>void,
    onEditRegion?:(regionId:number)=>void,
    onUpdateRegionBounds?:(regionId:number,start:number,end:number)=>void,
    onCreateRegionClick?:(time:number)=>void,
    onCreateRegionDrag?:(start:number,end:number)=>void,
    onCancelCreate?:()=>void,
    onCopyRegion?:(regionId:number)=>void,
    selectedRegionId?: number,
    createMode?: boolean,
    editingRegionBounds?: EditingRegionBounds,
    onRegionSelect?: (regionId: number) => void,
    onRegionDeselect?: () => void,
}


export function WaveformRenderer({
    regionSet,
    url,
    waveRef,
    playback,
    onRegionDetails,
    onUpdateRegionBounds,
    onCreateRegionDrag,
    onCancelCreate,
    selectedRegionId,
    createMode,
    editingRegionBounds,
    onRegionSelect,
    onRegionDeselect,
  }:WaveformRendererProps
  ){
    const waveformShellRef = useRef<HTMLDivElement | null>(null);
    const waveformRef = useRef<HTMLDivElement | null>(null);
    const regionSetRef = useRef<TrackRegionSetViewModel | undefined>(regionSet);
    const onRegionDetailsRef = useRef(onRegionDetails);
    const onUpdateRegionBoundsRef = useRef(onUpdateRegionBounds);
    const onCreateRegionDragRef = useRef(onCreateRegionDrag);
    const onRegionSelectRef = useRef(onRegionSelect);
    const onRegionDeselectRef = useRef(onRegionDeselect);
    const createModeRef = useRef(createMode ?? false);
    const editingRegionBoundsRef = useRef<EditingRegionBounds>(editingRegionBounds ?? null);
    const [regionsPlugin, setRegionsPlugin] = useState<RegionsPlugin | null>(null);
    const renderedRegionIds = useRef<Set<string>>(new Set());

    const beginPlaybackLoad = playback.beginLoading;
    const bindPlaybackWaveform = playback.bindWaveform;

    useEffect(() => {
        regionSetRef.current = regionSet;
    }, [regionSet]);

    useEffect(() => {
        onRegionDetailsRef.current = onRegionDetails;
    }, [onRegionDetails]);

    useEffect(() => {
        onUpdateRegionBoundsRef.current = onUpdateRegionBounds;
    }, [onUpdateRegionBounds]);

    useEffect(() => {
        onCreateRegionDragRef.current = onCreateRegionDrag;
    }, [onCreateRegionDrag]);

    useEffect(() => { onRegionSelectRef.current = onRegionSelect; }, [onRegionSelect]);
    useEffect(() => { onRegionDeselectRef.current = onRegionDeselect; }, [onRegionDeselect]);
    useEffect(() => { createModeRef.current = createMode ?? false; }, [createMode]);
    useEffect(() => { editingRegionBoundsRef.current = editingRegionBounds ?? null; }, [editingRegionBounds]);

    useEffect(() => {
        const waveformElement = waveformRef.current;
        const waveformShellElement = waveformShellRef.current;

        if (!waveformElement || !waveformShellElement || !url) {
            return;
        }

        beginPlaybackLoad();

        const { wave: waveform, regions } = createWaveFormPlayer(
            url,
            regionSetRef.current?.regions ?? [],
            waveformElement,
            (regionId) => onRegionDetailsRef.current?.(regionId),
            (regionId) => onRegionSelectRef.current?.(regionId),
            async (updatedRegion, side) => {
                const currentRegionSet = regionSetRef.current;
                if (!currentRegionSet?.regions) return;
                const editingBounds = editingRegionBoundsRef.current;
                if (!editingBounds || String(editingBounds.regionId) !== updatedRegion.id) {
                    const original = currentRegionSet.regions.find(
                        (region) => String(region.regionId) === updatedRegion.id,
                    );
                    if (original) {
                        updatedRegion.setOptions({ start: original.start, end: original.end });
                    }
                    return;
                }

                const original = currentRegionSet.regions.find(
                    (region) => String(region.regionId) === updatedRegion.id,
                );
                if (!original) return;

                const clamped = clampUpdatedRegionBounds(
                    updatedRegion.id,
                    updatedRegion.start,
                    updatedRegion.end,
                    currentRegionSet.regions,
                    waveform.getDuration(),
                    side,
                );

                if (clamped.start !== updatedRegion.start || clamped.end !== updatedRegion.end) {
                    updatedRegion.setOptions({ start: clamped.start, end: clamped.end });
                }

                if (!onUpdateRegionBoundsRef.current) return;
                onUpdateRegionBoundsRef.current(Number(updatedRegion.id), clamped.start, clamped.end);
            }
        );

        waveRef.current = waveform;
        const cleanupPlaybackBindings = bindPlaybackWaveform(waveform);
        setRegionsPlugin(regions);

        return () => {
            cleanupPlaybackBindings();
            waveform.destroy();
            waveRef.current = null;
            setRegionsPlugin(null);
        };
    }, [beginPlaybackLoad, bindPlaybackWaveform, url, waveRef]);

    useEffect(()=>{
        if(!regionsPlugin || !waveRef.current){
            return;
        }

        if(!regionSet){
            regionsPlugin.clearRegions();
            renderedRegionIds.current = new Set();
            return;
        }

        const displayedRegions = regionSet.regions.map((region) =>
            editingRegionBounds && editingRegionBounds.regionId === region.regionId
                ? { ...region, start: editingRegionBounds.start, end: editingRegionBounds.end }
                : region,
        );

        const currentIds=new Set(displayedRegions.map(r=>String(r.regionId)));
        const existingIds=renderedRegionIds.current;

         // Remove regions that no longer exist
        for(const id of existingIds){
            if(currentIds.has(id))
                continue;
            regionsPlugin.getRegions().find(x=>x.id===id)?.remove();
            existingIds.delete(id);

        }
        //add new regions
        for(const region of displayedRegions){
            const id=String(region.regionId);
            const existing=regionsPlugin.getRegions().find(x=>x.id===id);
            const isEditable = editingRegionBounds?.regionId === region.regionId;
            if(existing){
                existing.setOptions({
                    start: region.start,
                    end: region.end,
                    content: region.name,
                    drag: false,
                    resize: isEditable,
                    color: colorForRegion(region.regionId, region.regionId === selectedRegionId),
                });
            }else{
                  addRegion(regionsPlugin, region, region.regionId === selectedRegionId, isEditable);
            }


        }
        renderedRegionIds.current=currentIds;
    },[editingRegionBounds, regionSet, regionSet?.regions, regionsPlugin, selectedRegionId]);

    useEffect(() => {
        const waveformShellElement = waveformShellRef.current;
        const wave = waveRef.current;

        if (!waveformShellElement || !wave || !regionSet) {
            return;
        }

        const onPointerDown = (event: PointerEvent) => {
            if (event.button !== 0) return;
            if (createModeRef.current) return; // overlay handles it

            const time = clientXToTime(event.clientX, waveformShellElement, wave.getDuration());
            if (!isPointInsideRegion(time, regionSet.regions)) {
                onRegionDeselectRef.current?.();
            }
        };

        waveformShellElement.addEventListener("pointerdown", onPointerDown);
        return () => {
            waveformShellElement.removeEventListener("pointerdown", onPointerDown);
        };
    }, [regionSet, url, waveRef]);

    if (playback.error) {
        return (
            <div className="p-4 bg-red-100 border border-red-400 text-red-700 rounded">
                <p className="font-semibold">Error: {playback.error}</p>
                <div className="mt-2 text-sm">
                    <p>Debug info:</p>
                    <p>• URL: {url ? 'Present' : 'Missing'}</p>
                    <p>• Region Set: {regionSet?.name || 'Unknown'}</p>
                </div>
            </div>
        );
    }
    return (
  <div
    className="relative flex min-h-0 w-full flex-1 flex-col overflow-hidden"
    style={{ backgroundColor: "var(--bg-darker)" }}
  >
    {playback.isLoading && (
      <div className="absolute inset-0 z-10 flex items-center justify-center bg-black/25">
        <div className="text-gray-500">Loading waveform...</div>
      </div>
    )}
    <div
      ref={waveformShellRef}
      className="relative min-h-0 flex-1"
    >
      <div ref={waveformRef} className="h-full w-full" style={createMode ? { pointerEvents: "none" } : undefined} />
      {createMode && regionSet && playback.duration > 0 && (
        <CreateRegionOverlay
          regions={regionSet.regions}
          duration={playback.duration}
          containerRef={waveformShellRef}
          onConfirm={(start, end) => onCreateRegionDragRef.current?.(start, end)}
          onDiscard={onCancelCreate}
        />
      )}
    </div>
  </div>
);
}
