import type { Graph } from "../Graph/Graph";

export interface TrackRegion{
    regionId:number,
    regionSetId:number,
    name:string,
    start:number,
    end:number,
    graph:Graph|undefined
}