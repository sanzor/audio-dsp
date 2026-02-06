import type { TrackRegionViewModel } from "../Region/TrackRegionViewModel";
import type { TrackRegionSet } from "./TrackRegionSet";

export interface TrackRegionSetViewModel extends Omit<TrackRegionSet,'regions'>{
    regions:TrackRegionViewModel[]
}