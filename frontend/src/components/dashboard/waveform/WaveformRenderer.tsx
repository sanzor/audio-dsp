import { useEffect, useRef, useState } from "react"
import { Pause, Play, Square, Volume2 } from "lucide-react"
import WaveSurfer from "wavesurfer.js"
import RegionsPlugin, { type Region } from 'wavesurfer.js/dist/plugins/regions.esm.js'
import Minimap from 'wavesurfer.js/dist/plugins/minimap.esm.js'
import type { TrackRegionViewModel } from "@/domain/Region/TrackRegionViewModel";
import type { TrackRegionSetViewModel } from "@/domain/RegionSet/TrackRegionSetViewModel";
import { useUIStore, type RightClickContext } from "@/Stores/UIStore";
import { Button } from "@/components/ui/button";
import { Slider } from "@/components/ui/slider";

const PLAYBACK_RATES = [0.5, 0.75, 1, 1.25, 1.5, 2] as const;
const PLAYBACK_RATE_PRESETS = [1, 1.5, 2] as const;


export interface WaveformRendererProps{
    regionSet?: TrackRegionSetViewModel,
    url:string|null,
    onRegionDetails?:(regionId:string)=>void,
    onDeleteRegion?:(regionId:string)=>void,
    onEditRegion?:(regionId:string)=>void,
    onCreateRegionClick?:(time:number)=>void,
    onCreateRegionDrag?:(start:number,end:number)=>void,
    onCopyRegion?:(regionId:string)=>void
}


export function WaveformRenderer({
    regionSet,
    url
  }:WaveformRendererProps
  ){
    const waveRef = useRef<WaveSurfer | null>(null);
    const waveformRef = useRef<HTMLDivElement | null>(null);
    const [regionsPlugin, setRegionsPlugin] = useState<RegionsPlugin | null>(null);
    const renderedRegionIds = useRef<Set<string>>(new Set());

    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [isPlaying, setIsPlaying] = useState(false);
    const [currentTime, setCurrentTime] = useState(0);
    const [duration, setDuration] = useState(0);
    const [volume, setVolume] = useState(80);
    const [playbackRate, setPlaybackRate] = useState<number>(1);
    const openContextMenu=useUIStore(x=>x.openContextMenu);

    useEffect(() => {
        const waveformElement = waveformRef.current;

        if (!waveformElement || !url) {
            return;
        }

        const onContextMenu = (e: MouseEvent) => {
            e.preventDefault();
            if (!regionSet) return;
            const bounding = waveformElement.getBoundingClientRect();
            const x = e.clientX - bounding.left;
            const width = waveformElement.offsetWidth;
            const time = waveRef.current!.getDuration() * (x / width);
            openContextMenu({ type:'waveform_timeline',  x: e.clientX, y: e.clientY ,regionSetId:regionSet.id.toString(),time:time });
        };

        waveformElement.addEventListener('contextmenu', onContextMenu);

        setIsLoading(true);
        setError(null);

        const { wave: waveform, regions } = createWaveFormPlayer(
            url,
            regionSet?.regions ?? [],
            waveformElement,
            openContextMenu
        );

        waveRef.current = waveform;
        waveform.setVolume(volume / 100);
        waveform.setPlaybackRate(playbackRate);
        setRegionsPlugin(regions);
        setIsPlaying(false);
        setCurrentTime(0);
        setDuration(0);

        waveform.once('ready', () => {
            setIsLoading(false);
            setDuration(waveform.getDuration());
        });

        waveform.on('error', (err) => {
            console.error("Waveform error:", err);
            setError(`Failed to load audio: ${err}`);
            setIsLoading(false);
        });

        waveform.on('play', () => {
            setIsPlaying(true);
        });

        waveform.on('pause', () => {
            setIsPlaying(false);
        });

        waveform.on('finish', () => {
            setIsPlaying(false);
            setCurrentTime(waveform.getDuration());
        });

        waveform.on('timeupdate', (time) => {
            setCurrentTime(time);
        });

        return () => {
            waveformElement.removeEventListener('contextmenu', onContextMenu);
            waveform.destroy();
            waveRef.current = null;
            setRegionsPlugin(null);
        };
    }, [url, regionSet]);

    useEffect(()=>{
        if(!regionsPlugin||!regionSet)return;
        const currentIds=new Set(regionSet.regions.map(r=>r.regionId.toString()));
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
            const id=region.regionId.toString();
            const existing=regionsPlugin.getRegions().find(x=>x.id===id);
            if(existing){
                existing.start=region.start;
                existing.end=region.end;
                existing.setContent(region.name)
            }else{
                  addRegion(regionsPlugin,region);   
            }
           
           
        }
        renderedRegionIds.current=currentIds;
    },[regionSet,regionSet?.regions,regionsPlugin]);

    useEffect(() => {
        if (!waveRef.current) return;
        waveRef.current.setVolume(volume / 100);
    }, [volume]);

    useEffect(() => {
        if (!waveRef.current) return;
        waveRef.current.setPlaybackRate(playbackRate);
    }, [playbackRate]);

    const handlePlay = () => {
        waveRef.current?.play();
    };

    const handlePause = () => {
        waveRef.current?.pause();
    };

    const handleStop = () => {
        if (!waveRef.current) return;
        waveRef.current.stop();
        setIsPlaying(false);
        setCurrentTime(0);
    };

    const handlePlaybackRateStep = (direction: -1 | 1) => {
        const currentIndex = PLAYBACK_RATES.findIndex((rate) => rate === playbackRate);
        const safeIndex = currentIndex >= 0 ? currentIndex : PLAYBACK_RATES.indexOf(1);
        const nextIndex = Math.min(
            PLAYBACK_RATES.length - 1,
            Math.max(0, safeIndex + direction),
        );
        setPlaybackRate(PLAYBACK_RATES[nextIndex]);
    };


    if (error) {
        return (
            <div className="p-4 bg-red-100 border border-red-400 text-red-700 rounded">
                <p className="font-semibold">Error: {error}</p>
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
    className="relative flex h-full min-h-0 w-full flex-col overflow-hidden rounded-lg border shadow-lg"
    style={{ backgroundColor: "var(--bg-darker)", borderColor: "rgba(255,255,255,0.08)" }}
  >
    {isLoading && (
      <div className="absolute inset-0 z-10 flex items-center justify-center bg-black/25">
        <div className="text-gray-500">Loading waveform...</div>
      </div>
    )}
    <div ref={waveformRef} className="min-h-0 flex-1" />
    <div
      className="flex shrink-0 items-center justify-between gap-4 border-t px-4 py-3"
      style={{ backgroundColor: "var(--bg-darkest)", borderColor: "rgba(255,255,255,0.08)" }}
    >
      <div className="flex items-center gap-2">
        <Button
          type="button"
          size="icon"
          variant="outline"
          onClick={handleStop}
          disabled={!waveRef.current || isLoading}
          aria-label="Stop playback"
          className="border-white/10 bg-white/5 text-white hover:bg-white/10 hover:text-white"
        >
          <Square className="size-4 fill-current" />
        </Button>
        <Button
          type="button"
          size="icon"
          variant="outline"
          onClick={handlePause}
          disabled={!waveRef.current || isLoading || !isPlaying}
          aria-label="Pause playback"
          className="border-white/10 bg-white/5 text-white hover:bg-white/10 hover:text-white"
        >
          <Pause className="size-4 fill-current" />
        </Button>
        <Button
          type="button"
          size="icon"
          onClick={handlePlay}
          disabled={!waveRef.current || isLoading}
          aria-label="Play waveform"
          className="bg-[var(--accent-blue)] text-white shadow-none hover:bg-[var(--accent-blue)]/90"
        >
          <Play className="size-4 fill-current" />
        </Button>
        <div className="min-w-28 text-xs tabular-nums" style={{ color: "var(--text-muted)" }}>
          {formatTime(currentTime)} / {formatTime(duration)}
        </div>
      </div>

      <div className="flex items-center gap-6">
        <div className="flex items-center gap-2">
          <Button
            type="button"
            size="sm"
            variant="outline"
            onClick={() => handlePlaybackRateStep(-1)}
            disabled={!waveRef.current || isLoading || playbackRate <= PLAYBACK_RATES[0]}
            aria-label="Decrease playback speed"
            className="h-8 border-white/10 bg-white/5 px-2 text-white hover:bg-white/10 hover:text-white"
          >
            -
          </Button>
          {PLAYBACK_RATE_PRESETS.map((rate) => (
            <Button
              key={rate}
              type="button"
              size="sm"
              variant={playbackRate === rate ? "default" : "outline"}
              onClick={() => setPlaybackRate(rate)}
              disabled={!waveRef.current || isLoading}
              aria-label={`Set playback speed to ${rate}x`}
              className={
                playbackRate === rate
                  ? "h-8 bg-[var(--accent-blue)] px-2.5 text-white shadow-none hover:bg-[var(--accent-blue)]/90"
                  : "h-8 border-white/10 bg-white/5 px-2.5 text-white hover:bg-white/10 hover:text-white"
              }
            >
              {rate}x
            </Button>
          ))}
          <Button
            type="button"
            size="sm"
            variant="outline"
            onClick={() => handlePlaybackRateStep(1)}
            disabled={!waveRef.current || isLoading || playbackRate >= PLAYBACK_RATES[PLAYBACK_RATES.length - 1]}
            aria-label="Increase playback speed"
            className="h-8 border-white/10 bg-white/5 px-2 text-white hover:bg-white/10 hover:text-white"
          >
            +
          </Button>
        </div>

        <div className="flex min-w-40 items-center gap-3">
          <Volume2 className="size-4" style={{ color: "var(--text-muted)" }} />
          <Slider
            min={0}
            max={100}
            step={1}
            value={[volume]}
            onValueChange={(value) => setVolume(value[0] ?? 0)}
            aria-label="Volume"
            className="w-32"
          />
        </div>
      </div>
    </div>
  </div>
);
}


// eslint-disable-next-line react-refresh/only-export-components
export function createWaveFormPlayer(
    url:string,
    trackRegions:TrackRegionViewModel[],
    container:HTMLElement,
    openContextMenu:(context: RightClickContext) => void)
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
        openContextMenu({type:'waveform_region',regionId:region.id!.toString(),x:e.clientX,y:e.clientY})
        region.play(true);
        region.setOptions({color:randomColor()});
    })
    regions.enableDragSelection({
        color:'rgba(255,0,0,1)'
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
const random = (min:number, max:number) => Math.random() * (max - min) + min
const randomColor = () => `rgba(${random(0, 255)}, ${random(0, 255)}, ${random(0, 255)}, 0.5)`

function formatTime(timeSeconds: number): string {
    if (!Number.isFinite(timeSeconds)) return "00:00";
    const totalSeconds = Math.max(0, Math.floor(timeSeconds));
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;
    return `${minutes.toString().padStart(2, "0")}:${seconds.toString().padStart(2, "0")}`;
}


function addRegions(regions:TrackRegionViewModel[],regionsObj:RegionsPlugin):RegionsPlugin{
    for(const elem of regions){
        addRegion(regionsObj,elem);
    }
    return regionsObj;
}

function addRegion(regionsObj:RegionsPlugin,elem:TrackRegionViewModel):RegionsPlugin{
    regionsObj.addRegion({
            id:String(elem.regionId),
            start:elem.start,
            end:elem.end,
            drag: true,
            resize: true,
            content:elem.name,
            color:randomColor()
        });
    return regionsObj;
}
