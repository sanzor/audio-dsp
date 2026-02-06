import type { TrackRegion } from "./TrackRegion";

export interface NormalizedTrackRegion extends Omit<TrackRegion, 'graph'> {
    graphId: string | null;
}
