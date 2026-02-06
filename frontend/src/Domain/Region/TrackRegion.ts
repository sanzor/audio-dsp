import type { Graph } from "../Graph/Graph";

export interface TrackRegion{
    regionId:string,
    regionSetId:string,
    name:string,
    start:number,
    end:number,
    graph:Graph|undefined
}