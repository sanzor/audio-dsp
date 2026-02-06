import type { TrackRegionSet } from "@/Domain/RegionSet/TrackRegionSet";

export interface GetRegionSetsForTrackResult{
    trackId:string,
    sets:TrackRegionSet[]
}