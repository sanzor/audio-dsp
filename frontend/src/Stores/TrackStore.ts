import type { NormalizedTrackMeta } from '@/domain/Track/NormalizedTrackMeta';
import { create, type StoreApi, type UseBoundStore } from 'zustand';

type TrackCache = Map<number, NormalizedTrackMeta>;

interface TrackState {
    tracks: TrackCache;
    loading: boolean;
}

interface TrackActions {
    getTrack: (trackId: number) => NormalizedTrackMeta | undefined;
    addTrack: (track: NormalizedTrackMeta) => void;
    removeTrack: (trackId: number) => void;
    updateTrack: (trackId: number, trackMeta: Partial<NormalizedTrackMeta>) => void;
    setAllTracks: (tracks: NormalizedTrackMeta[]) => void;
    attachRegionSet: (trackId: number, regionSetId: number) => void;
    detachRegionSet: (trackId: number, regionSetId: number) => void;
}

type TrackStore = TrackState & TrackActions;

const updateTrackRegionSets = (
    track: NormalizedTrackMeta,
    updater: (regionSetIds: number[]) => number[]
): NormalizedTrackMeta => {
    const currentIds = track.region_sets_ids ?? [];
    return {
        ...track,
        region_sets_ids: updater(currentIds),
    };
};

export const useTrackStore: UseBoundStore<StoreApi<TrackStore>> = create<TrackStore>((set, get) => ({
    tracks: new Map(),
    loading: true,

    setAllTracks: (newTracks: NormalizedTrackMeta[]) => {
        const trackMap = new Map<number, NormalizedTrackMeta>();
        newTracks.forEach((t) => trackMap.set(t.trackId, t));
        set({
            tracks: trackMap,
            loading: false,
        });
    },

    getTrack: (trackId: number): NormalizedTrackMeta | undefined => {
        return get().tracks.get(trackId);
    },

    addTrack: (track: NormalizedTrackMeta): void => {
        set((state: TrackState) => {
            const newMap = new Map(state.tracks);
            newMap.set(track.trackId, track);
            return { tracks: newMap };
        });
    },

    removeTrack: (trackId: number): void => {
        set((state: TrackState) => {
            const newMap = new Map(state.tracks);
            newMap.delete(trackId);
            return { tracks: newMap };
        });
    },

    updateTrack: (trackId: number, updates: Partial<NormalizedTrackMeta>): void => {
        set((state: TrackState) => {
            const trackToUpdate = state.tracks.get(trackId);
            if (!trackToUpdate) return state;

            const newMap = new Map(state.tracks);
            newMap.set(trackId, {
                ...trackToUpdate,
                ...updates,
            });

            return { tracks: newMap };
        });
    },

    attachRegionSet: (trackId: number, regionSetId: number) => {
        set((state: TrackState) => {
            const track = state.tracks.get(trackId);
            if (!track) return state;

            const updatedTrack = updateTrackRegionSets(track, (ids) =>
                ids.includes(regionSetId) ? ids : [...ids, regionSetId]
            );

            const newMap = new Map(state.tracks);
            newMap.set(trackId, updatedTrack);
            return { tracks: newMap };
        });
    },

    detachRegionSet: (trackId: number, regionSetId: number) => {
        set((state: TrackState) => {
            const track = state.tracks.get(trackId);
            if (!track) return state;

            const updatedTrack = updateTrackRegionSets(track, (ids) =>
                ids.filter((id) => id !== regionSetId)
            );

            const newMap = new Map(state.tracks);
            newMap.set(trackId, updatedTrack);
            return { tracks: newMap };
        });
    },
}));
