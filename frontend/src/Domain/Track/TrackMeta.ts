import type { TrackRegionSet } from "../RegionSet/TrackRegionSet";
import type { TrackInfo } from "./TrackInfo";

export interface TrackMeta{
    trackInfo:TrackInfo,
    trackId:string
    regionSets:TrackRegionSet[]
}