import type { TrackRegion } from "../Region/TrackRegion";


export interface TrackRegionSet{
    id:number,
    trackId:number,
    name:string,
    regions: TrackRegion[]
}