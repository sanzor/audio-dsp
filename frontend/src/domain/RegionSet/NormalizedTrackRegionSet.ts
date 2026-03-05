import type { TrackRegionSet } from "./TrackRegionSet";

export interface NormalizedTrackRegionSet extends Omit<TrackRegionSet,'regions'>{
    region_ids:string[]
}