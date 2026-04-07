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
    setTrackRegionSets: (trackId: number, regionSetIds: number[]) => void;
    attachRegionSet: (trackId: number, regionSetId: number) => void;
    detachRegionSet: (trackId: number, regionSetId: number) => void;
    clear: () => void;
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

const resolveTrackRegionSetIds = (
    incomingTrack: NormalizedTrackMeta,
    existingTrack?: NormalizedTrackMeta
): number[] => {
    if (incomingTrack.region_sets_ids.length > 0) {
        return incomingTrack.region_sets_ids;
    }

    return existingTrack?.region_sets_ids ?? [];
};

export const useTrackStore: UseBoundStore<StoreApi<TrackStore>> = create<TrackStore>((set, get) => ({
    tracks: new Map(),
    loading: true,

    setAllTracks: (newTracks: NormalizedTrackMeta[]) => {
        const trackMap = new Map<number, NormalizedTrackMeta>();
        const existingTracks = get().tracks;

        newTracks.forEach((track) => {
            const existingTrack = existingTracks.get(track.trackId);
            trackMap.set(track.trackId, {
                ...track,
                region_sets_ids: resolveTrackRegionSetIds(track, existingTrack),
            });
        });

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
            const existingTrack = state.tracks.get(track.trackId);
            newMap.set(track.trackId, {
                ...track,
                region_sets_ids: resolveTrackRegionSetIds(track, existingTrack),
            });
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

    setTrackRegionSets: (trackId: number, regionSetIds: number[]) => {
        set((state: TrackState) => {
            const track = state.tracks.get(trackId);
            if (!track) return state;

            const newMap = new Map(state.tracks);
            newMap.set(trackId, {
                ...track,
                region_sets_ids: [...regionSetIds],
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

    clear: () => set({ tracks: new Map(), loading: true }),

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
