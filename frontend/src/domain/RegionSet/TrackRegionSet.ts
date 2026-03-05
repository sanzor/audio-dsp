import type { TrackRegion } from "../Region/TrackRegion";


export interface TrackRegionSet{
    id:string,
    trackId:string,
    name:string,
    regions: TrackRegion[]
}