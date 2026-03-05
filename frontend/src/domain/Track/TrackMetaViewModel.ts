
import type { TrackRegionSetViewModel } from "../RegionSet/TrackRegionSetViewModel";
import type { TrackMeta } from "./TrackMeta";

export interface TrackMetaViewModel extends Omit<TrackMeta,"regionSets"> {
    regionSets: TrackRegionSetViewModel[];
}
