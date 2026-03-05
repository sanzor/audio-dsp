import type { Graph } from "../Graph/Graph";
import type { TrackRegion } from "./TrackRegion";

export interface TrackRegionViewModel extends Omit<TrackRegion, "graph"> {
    graph: Graph | null;
}
