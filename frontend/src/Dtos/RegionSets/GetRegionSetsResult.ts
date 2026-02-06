import type { TrackRegionSet } from "@/Domain/RegionSet/TrackRegionSet";

export interface GetRegionSetsResult{
    track_region_sets_map:Map<string,TrackRegionSet[]>
}