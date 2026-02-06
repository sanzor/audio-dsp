import type { NormalizedTrackRegion } from '@/Domain/Region/NormalizedTrackRegion';
import { create, type StoreApi, type UseBoundStore } from 'zustand';

type RegionCache = Map<string, NormalizedTrackRegion>;

interface RegionState {
    regions: RegionCache;
    loading: boolean;
}

interface RegionActions {
    getRegion: (regionId: string) => NormalizedTrackRegion | undefined;
    addRegion: (region: NormalizedTrackRegion) => void;
    removeRegion: (regionId: string) => void;
    updateRegion: (regionId: string, region: Partial<NormalizedTrackRegion>) => void;
    setAllRegions: (regions: NormalizedTrackRegion[]) => void;
    removeRegionsBySetId: (setId: string) => void;
    // Graph relationship (single value, not array)
    setGraph: (regionId: string, graphId: string) => void;
    clearGraph: (regionId: string) => void;
}

type RegionStore = RegionState & RegionActions;

export const useRegionStore: UseBoundStore<StoreApi<RegionStore>> = create<RegionStore>((set, get) => ({
    regions: new Map(),
    loading: true,

    setAllRegions: (newRegions: NormalizedTrackRegion[]) => {
        const regionMap = new Map<string, NormalizedTrackRegion>();
        newRegions.forEach((t) => regionMap.set(t.regionId, t));
        set({
            regions: regionMap,
            loading: false,
        });
    },

    getRegion: (regionId: string): NormalizedTrackRegion | undefined => {
        return get().regions.get(regionId);
    },

    addRegion: (region: NormalizedTrackRegion): void => {
        set((state: RegionState) => {
            const newMap = new Map(state.regions);
            newMap.set(region.regionId, region);
            return { regions: newMap };
        });
    },

    removeRegion: (regionId: string): void => {
        set((state: RegionState) => {
            const newMap = new Map(state.regions);
            newMap.delete(regionId);
            return { regions: newMap };
        });
    },

    removeRegionsBySetId: (setId: string): void => {
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

    updateRegion: (regionId: string, updates: Partial<NormalizedTrackRegion>): void => {
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

    setGraph: (regionId: string, graphId: string) =>
        set((state: RegionState) => {
            const region = state.regions.get(regionId);
            if (!region) return state;

            const newMap = new Map(state.regions);
            newMap.set(regionId, { ...region, graphId });
            return { regions: newMap };
        }),

    clearGraph: (regionId: string) =>
        set((state: RegionState) => {
            const region = state.regions.get(regionId);
            if (!region) return state;

            const newMap = new Map(state.regions);
            newMap.set(regionId, { ...region, graphId: null });
            return { regions: newMap };
        }),
}));
