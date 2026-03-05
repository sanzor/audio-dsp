import type { TrackMeta } from "./TrackMeta";

export interface NormalizedTrackMeta  extends Omit<TrackMeta,'regionSets'>{
    region_sets_ids:string[]
}