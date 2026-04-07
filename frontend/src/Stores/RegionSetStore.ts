// /src/Stores/useRegionSetStore.ts

import type { NormalizedTrackRegionSet } from '@/domain/RegionSet/NormalizedTrackRegionSet';
import { create, type StoreApi, type UseBoundStore } from 'zustand';

type RegionSetCache = Map<number, NormalizedTrackRegionSet>;

interface RegionSetState {
    regionSets: RegionSetCache;
    loading: boolean;
}

interface RegionSetActions {
    getRegionSet: (setId: number) => NormalizedTrackRegionSet | undefined;
    setAllRegionSets: (sets: NormalizedTrackRegionSet[]) => void;
    addRegionSet: (set: NormalizedTrackRegionSet) => void;
    removeRegionSet: (setId: number) => void;
    updateRegionSet: (setId: number, updates: Partial<NormalizedTrackRegionSet>) => void;
    attachRegion: (setId: number, regionId: number) => void;
    detachRegion: (setId: number, regionId: number) => void;
    clear: () => void;
}

type RegionSetStore = RegionSetState & RegionSetActions;

const updateRegionIds = (
    entity: NormalizedTrackRegionSet,
    updater: (regionIds: number[]) => number[]
): NormalizedTrackRegionSet => ({
    ...entity,
    region_ids: updater(entity.region_ids ?? []),
});

export const useRegionSetStore: UseBoundStore<StoreApi<RegionSetStore>> = create<RegionSetStore>((set, get) => ({
    regionSets: new Map(),
    loading: true,

    setAllRegionSets: (newRegionSets: NormalizedTrackRegionSet[]) => {
        const setMap = new Map<number, NormalizedTrackRegionSet>();
        newRegionSets.forEach((s) => setMap.set(s.id, s));
        set({ regionSets: setMap, loading: false });
    },

    getRegionSet: (setId: number) => get().regionSets.get(setId),

    addRegionSet: (setToAdd: NormalizedTrackRegionSet) =>
        set((state: RegionSetState) => {
            const newMap = new Map(state.regionSets);
            newMap.set(setToAdd.id, setToAdd);
            return { regionSets: newMap };
        }),

    removeRegionSet: (setId: number) =>
        set((state: RegionSetState) => {
            const newMap = new Map(state.regionSets);
            newMap.delete(setId);
            return { regionSets: newMap };
        }),

    updateRegionSet: (setId: number, updates: Partial<NormalizedTrackRegionSet>) =>
        set((state: RegionSetState) => {
            const setEntity = state.regionSets.get(setId);
            if (!setEntity) return state;

            const newMap = new Map(state.regionSets);
            newMap.set(setId, { ...setEntity, ...updates });
            return { regionSets: newMap };
        }),

    clear: () => set({ regionSets: new Map(), loading: true }),

    attachRegion: (setId: number, regionId: number) =>
        set((state: RegionSetState) => {
            const setEntity = state.regionSets.get(setId);
            if (!setEntity) return state;

            const updated = updateRegionIds(setEntity, (ids) =>
                ids.includes(regionId) ? ids : [...ids, regionId]
            );

            const newMap = new Map(state.regionSets);
            newMap.set(setId, updated);
            return { regionSets: newMap };
        }),

    detachRegion: (setId: number, regionId: number) =>
        set((state: RegionSetState) => {
            const setEntity = state.regionSets.get(setId);
            if (!setEntity) return state;

            const updated = updateRegionIds(setEntity, (ids) => ids.filter((id) => id !== regionId));

            const newMap = new Map(state.regionSets);
            newMap.set(setId, updated);
            return { regionSets: newMap };
        }),
}));
