import { useEffect, useRef, useState } from "react"
import WaveSurfer from "wavesurfer.js"
import RegionsPlugin, { type Region } from 'wavesurfer.js/dist/plugins/regions.esm.js'
import Minimap from 'wavesurfer.js/dist/plugins/minimap.esm.js'
import type { TrackRegionViewModel } from "@/domain/Region/TrackRegionViewModel";
import type { TrackRegionSetViewModel } from "@/domain/RegionSet/TrackRegionSetViewModel";
import { PlaybackControls } from "./PlaybackControls";
import { usePlaybackController } from "./usePlaybackController";
import { CreateRegionOverlay } from "./CreateRegionOverlay";

const REGION_GAP = 0.05;

export interface WaveformRendererProps{
    regionSet?: TrackRegionSetViewModel,
    url:string|null,
    onRegionDetails?:(regionId:number)=>void,
    onDeleteRegion?:(regionId:number)=>void,
    onEditRegion?:(regionId:number)=>void,
    onUpdateRegionBounds?:(regionId:number,start:number,end:number)=>Promise<void> | void,
    onCreateRegionClick?:(time:number)=>void,
    onCreateRegionDrag?:(start:number,end:number)=>void,
    onCopyRegion?:(regionId:number)=>void,
    selectedRegionId?: number,
    createMode?: boolean,
    onRegionSelect?: (regionId: number) => void,
    onRegionDeselect?: () => void,
}


export function WaveformRenderer({
    regionSet,
    url,
    onRegionDetails,
    onUpdateRegionBounds,
    onCreateRegionDrag,
    selectedRegionId,
    createMode,
    onRegionSelect,
    onRegionDeselect,
  }:WaveformRendererProps
  ){
    const waveRef = useRef<WaveSurfer | null>(null);
    const waveformShellRef = useRef<HTMLDivElement | null>(null);
    const waveformRef = useRef<HTMLDivElement | null>(null);
    const regionSetRef = useRef<TrackRegionSetViewModel | undefined>(regionSet);
    const onRegionDetailsRef = useRef(onRegionDetails);
    const onUpdateRegionBoundsRef = useRef(onUpdateRegionBounds);
    const onCreateRegionDragRef = useRef(onCreateRegionDrag);
    const onRegionSelectRef = useRef(onRegionSelect);
    const onRegionDeselectRef = useRef(onRegionDeselect);
    const createModeRef = useRef(createMode ?? false);
    const [regionsPlugin, setRegionsPlugin] = useState<RegionsPlugin | null>(null);
    const renderedRegionIds = useRef<Set<string>>(new Set());

    const playback = usePlaybackController(waveRef);
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

                try {
                    await onUpdateRegionBoundsRef.current(Number(updatedRegion.id), clamped.start, clamped.end);
                } catch (regionUpdateError) {
                    console.error("Failed to persist region bounds:", regionUpdateError);
                    updatedRegion.setOptions({ start: original.start, end: original.end });
                }
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
    }, [beginPlaybackLoad, bindPlaybackWaveform, url]);

    useEffect(()=>{
        if(!regionsPlugin){
            return;
        }

        if(!regionSet){
            regionsPlugin.clearRegions();
            renderedRegionIds.current = new Set();
            return;
        }

        const currentIds=new Set(regionSet.regions.map(r=>String(r.regionId)));
        const existingIds=renderedRegionIds.current;

         // Remove regions that no longer exist
        for(const id of existingIds){
            if(currentIds.has(id))
                continue;
            regionsPlugin.getRegions().find(x=>x.id===id)?.remove();
            existingIds.delete(id);

        }
        //add new regions
        for(const region of regionSet.regions){
            const id=String(region.regionId);
            const existing=regionsPlugin.getRegions().find(x=>x.id===id);
            if(existing){
                existing.setOptions({
                    start: region.start,
                    end: region.end,
                    content: region.name,
                    drag: true,
                    resize: true,
                    color: colorForRegion(region.regionId, region.regionId === selectedRegionId),
                });
            }else{
                  addRegion(regionsPlugin, region, region.regionId === selectedRegionId);
            }


        }
        renderedRegionIds.current=currentIds;
    },[regionSet,regionSet?.regions,regionsPlugin, selectedRegionId]);

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
    }, [regionSet, url]);

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
    className="relative flex flex-1 min-h-0 w-full flex-col overflow-hidden rounded-lg border shadow-lg"
    style={{ backgroundColor: "var(--bg-darker)", borderColor: "rgba(255,255,255,0.08)" }}
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
      <div ref={waveformRef} className="h-full w-full" />
      {createMode && regionSet && playback.duration > 0 && (
        <CreateRegionOverlay
          regions={regionSet.regions}
          duration={playback.duration}
          containerRef={waveformShellRef}
          onConfirm={(start, end) => onCreateRegionDragRef.current?.(start, end)}
        />
      )}
    </div>
    <PlaybackControls
      hasWaveform={waveRef.current !== null}
      isLoading={playback.isLoading}
      isPlaying={playback.isPlaying}
      currentTime={playback.currentTime}
      duration={playback.duration}
      volume={playback.volume}
      playbackRate={playback.playbackRate}
      onPlay={playback.play}
      onPause={playback.pause}
      onStop={playback.stop}
      onVolumeChange={playback.setVolume}
      onPlaybackRateChange={playback.setPlaybackRate}
      onPlaybackRateStep={playback.stepPlaybackRate}
    />
  </div>
);
}


// eslint-disable-next-line react-refresh/only-export-components
export function createWaveFormPlayer(
    url:string,
    trackRegions:TrackRegionViewModel[],
    container:HTMLElement,
    onRegionDetails?:(regionId:number)=>void,
    onRegionSelect?:(regionId:number)=>void,
    onRegionUpdated?: (region: Region, side?: "start" | "end") => Promise<void> | void)
    :{wave:WaveSurfer,regions:RegionsPlugin}{
    let activeRegion:Region|null=null;
    const regions = RegionsPlugin.create();
    regions.on('region-in',(region)=>{
        activeRegion=region;
    });
    regions.on('region-out',(region)=>{
        if(activeRegion===region){
            console.log("some");
        }
        activeRegion=null;
    });
    regions.on('region-clicked',(region,e)=>{
        e.preventDefault();
        e.stopImmediatePropagation();
        activeRegion=region;
        region.play(true);
        onRegionSelect?.(Number(region.id));
    })
    regions.on('region-double-clicked', (region, e) => {
        e.preventDefault();
        e.stopImmediatePropagation();
        onRegionDetails?.(Number(region.id));
    });
    regions.on('region-updated', (region, side) => {
        void onRegionUpdated?.(region, side);
    });
    const wave=WaveSurfer.create({
        container:container,
        waveColor:'rgb(100, 152, 200)',
        progressColor:'rgb(100,100,100)',
        url:url,
        plugins:[
              regions,
              Minimap.create({
                    height: 20,
                    waveColor: '#ddd',
                    progressColor: '#999',
      // the Minimap takes all the same options as the WaveSurfer itself
              }),
        ],
        mediaControls:false
    });
    wave.on('interaction',()=>{
        activeRegion=null;
    });
    wave.once('ready',()=>{
        addRegions(trackRegions,regions);
    });

    return {wave:wave,regions:regions}
}

function clientXToTime(clientX: number, element: HTMLElement, totalDuration: number): number {
    const { left, width } = element.getBoundingClientRect();
    if (width <= 0 || totalDuration <= 0) return 0;
    const ratio = Math.min(1, Math.max(0, (clientX - left) / width));
    return ratio * totalDuration;
}

function isPointInsideRegion(time: number, regions: TrackRegionViewModel[]): boolean {
    return regions.some((region) => time >= region.start && time <= region.end);
}


function clampUpdatedRegionBounds(
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


function addRegions(regions:TrackRegionViewModel[],regionsObj:RegionsPlugin):RegionsPlugin{
    for(const elem of regions){
        addRegion(regionsObj,elem);
    }
    return regionsObj;
}

function addRegion(regionsObj:RegionsPlugin, elem:TrackRegionViewModel, isSelected = false):RegionsPlugin{
    regionsObj.addRegion({
            id:String(elem.regionId),
            start:elem.start,
            end:elem.end,
            drag: true,
            resize: true,
            content:elem.name,
            color:colorForRegion(elem.regionId, isSelected)
        });
    return regionsObj;
}

function colorForRegion(regionId: string | number, isSelected = false): string {
    const value = String(regionId);
    let hash = 0;

    for (let index = 0; index < value.length; index += 1) {
        hash = (hash * 31 + value.charCodeAt(index)) >>> 0;
    }

    const hue = hash % 360;
    return isSelected
        ? `hsla(${hue}, 90%, 65%, 0.65)`
        : `hsla(${hue}, 72%, 58%, 0.32)`;
}
