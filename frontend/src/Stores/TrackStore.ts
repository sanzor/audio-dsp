import type { NormalizedTrackMeta } from '@/Domain/Track/NormalizedTrackMeta';
import { create, type StoreApi, type UseBoundStore } from 'zustand';

type TrackCache = Map<string, NormalizedTrackMeta>;

interface TrackState {
    tracks: TrackCache;
    loading: boolean;
}

interface TrackActions {
    getTrack: (trackId: string) => NormalizedTrackMeta | undefined;
    addTrack: (track: NormalizedTrackMeta) => void;
    removeTrack: (trackId: string) => void;
    updateTrack: (trackId: string, trackMeta: Partial<NormalizedTrackMeta>) => void;
    setAllTracks: (tracks: NormalizedTrackMeta[]) => void;
    attachRegionSet: (trackId: string, regionSetId: string) => void;
    detachRegionSet: (trackId: string, regionSetId: string) => void;
}

type TrackStore = TrackState & TrackActions;

const updateTrackRegionSets = (
    track: NormalizedTrackMeta,
    updater: (regionSetIds: string[]) => string[]
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
        const trackMap = new Map<string, NormalizedTrackMeta>();
        newTracks.forEach((t) => trackMap.set(t.trackId, t));
        set({
            tracks: trackMap,
            loading: false,
        });
    },

    getTrack: (trackId: string): NormalizedTrackMeta | undefined => {
        return get().tracks.get(trackId);
    },

    addTrack: (track: NormalizedTrackMeta): void => {
        set((state: TrackState) => {
            const newMap = new Map(state.tracks);
            newMap.set(track.trackId, track);
            return { tracks: newMap };
        });
    },

    removeTrack: (trackId: string): void => {
        set((state: TrackState) => {
            const newMap = new Map(state.tracks);
            newMap.delete(trackId);
            return { tracks: newMap };
        });
    },

    updateTrack: (trackId: string, updates: Partial<NormalizedTrackMeta>): void => {
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

    attachRegionSet: (trackId: string, regionSetId: string) => {
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

    detachRegionSet: (trackId: string, regionSetId: string) => {
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
