import type { NormalizedTrackRegion } from '@/domain/Region/NormalizedTrackRegion';
import { create, type StoreApi, type UseBoundStore } from 'zustand';

type RegionCache = Map<number, NormalizedTrackRegion>;

interface RegionState {
    regions: RegionCache;
    loading: boolean;
}

interface RegionActions {
    getRegion: (regionId: number) => NormalizedTrackRegion | undefined;
    addRegion: (region: NormalizedTrackRegion) => void;
    removeRegion: (regionId: number) => void;
    updateRegion: (regionId: number, region: Partial<NormalizedTrackRegion>) => void;
    setAllRegions: (regions: NormalizedTrackRegion[]) => void;
    removeRegionsBySetId: (setId: number) => void;
    replaceRegionsBySetId: (setId: number, regions: NormalizedTrackRegion[]) => void;
    // Graph relationship (single value, not array)
    setGraph: (regionId: number, graphId: number) => void;
    clearGraph: (regionId: number) => void;
    clear: () => void;
}

type RegionStore = RegionState & RegionActions;

export const useRegionStore: UseBoundStore<StoreApi<RegionStore>> = create<RegionStore>((set, get) => ({
    regions: new Map(),
    loading: true,

    setAllRegions: (newRegions: NormalizedTrackRegion[]) => {
        const regionMap = new Map<number, NormalizedTrackRegion>();
        newRegions.forEach((t) => regionMap.set(t.regionId, t));
        set({
            regions: regionMap,
            loading: false,
        });
    },

    getRegion: (regionId: number): NormalizedTrackRegion | undefined => {
        return get().regions.get(regionId);
    },

    addRegion: (region: NormalizedTrackRegion): void => {
        set((state: RegionState) => {
            const newMap = new Map(state.regions);
            newMap.set(region.regionId, region);
            return { regions: newMap };
        });
    },

    removeRegion: (regionId: number): void => {
        set((state: RegionState) => {
            const newMap = new Map(state.regions);
            newMap.delete(regionId);
            return { regions: newMap };
        });
    },

    removeRegionsBySetId: (setId: number): void => {
        set((state: RegionState) => {
            const newMap = new Map(state.regions);
            for (const [key, value] of newMap) {
                if (value.regionSetId === setId) {
                    newMap.delete(key);
                }
            }
            return { regions: newMap };
        });
    },

    replaceRegionsBySetId: (setId: number, regions: NormalizedTrackRegion[]): void => {
        set((state: RegionState) => {
            const newMap = new Map(state.regions);
            for (const [key, value] of newMap) {
                if (value.regionSetId === setId) newMap.delete(key);
            }
            for (const region of regions) {
                newMap.set(region.regionId, region);
            }
            return { regions: newMap };
        });
    },

    updateRegion: (regionId: number, updates: Partial<NormalizedTrackRegion>): void => {
        set((state: RegionState) => {
            const trackToUpdate = state.regions.get(regionId);
            if (!trackToUpdate) return state;

            const newMap = new Map(state.regions);
            newMap.set(regionId, {
                ...trackToUpdate,
                ...updates,
            });

            return { regions: newMap };
        });
    },

    setGraph: (regionId: number, graphId: number) =>
        set((state: RegionState) => {
            const region = state.regions.get(regionId);
            if (!region) return state;

            const newMap = new Map(state.regions);
            newMap.set(regionId, { ...region, graphId });
            return { regions: newMap };
        }),

    clearGraph: (regionId: number) =>
        set((state: RegionState) => {
            const region = state.regions.get(regionId);
            if (!region) return state;

            const newMap = new Map(state.regions);
            newMap.set(regionId, { ...region, graphId: null });
            return { regions: newMap };
        }),

    clear: () => set({ regions: new Map(), loading: true }),
}));
