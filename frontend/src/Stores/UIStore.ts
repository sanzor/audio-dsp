import { create, type StoreApi, type UseBoundStore } from 'zustand';
import { devtools, subscribeWithSelector } from 'zustand/middleware';
import type { PasteGraphParams, PasteRegionParams, PasteRegionSetParams } from './PasteParams';
import type { CanonicalAudio } from '@/Audio/CanonicalAudio';
import { useRegionSetStore } from './RegionSetStore';
import { useRegionStore } from './RegionStore';
import { useGraphStore } from './GraphStore';

export type RightClickContext =
  | { type: 'track'; trackId: number; x: number; y: number }
  | { type: 'region'; regionId: number; x: number; y: number }
  | { type: 'regionSet'; regionSetId: number; x: number; y: number }
  | { type: 'graph'; graphId: number; x: number; y: number }
  | null;

export type ActiveSelection = {
  trackId: number | null;
  regionSetId: number | null;
  regionId: number | null;
  graphId: number | null;
};

export type Clipboard =
  | { type: 'track'; trackId: number }
  | { type: 'regionSet'; regionSetId: number }
  | { type: 'region'; regionId: number }
  | { type: 'graph'; graphId: number }
  | { type: 'node'; nodeId: number }
  | { type: 'edge'; edgeId: number }
  | null;

export type ModalState =
  // Track modals
  | { type: 'createTrack'; canonicalAudio: CanonicalAudio | null }
  | { type: 'renameTrack'; trackId: number }
  | { type: 'detailsTrack'; trackId: number }
  | { type: 'deleteTrack'; trackId: number }
  | { type: 'pasteTrack'; trackId: number }
  // RegionSet modals
  | { type: 'createRegionSet'; trackId: number }
  | { type: 'renameRegionSet'; regionSetId: number }
  | { type: 'detailsRegionSet'; regionSetId: number }
  | { type: 'deleteRegionSet'; regionSetId: number }
  | { type: 'pasteRegionSet'; params: PasteRegionSetParams }
  // Region modals
  | { type: 'createRegion'; regionSetId: number; startTime?: number; endTime?: number }
  | { type: 'renameRegion'; regionId: number }
  | { type: 'detailsRegion'; regionId: number }
  | { type: 'pasteRegion'; params: PasteRegionParams }
  | { type: 'deleteRegion'; regionId: number }
  // Graph modals
  | { type: 'createGraph'; regionId: number }
  | { type: 'renameGraph'; graphId: number }
  | { type: 'detailsGraph'; graphId: number }
  | { type: 'deleteGraph'; graphId: number }
  | { type: 'pasteGraph'; params: PasteGraphParams }
  | null;

export type UIStore = {
  activeSelection: ActiveSelection;
  clipboard: Clipboard;
  rightClickContext: RightClickContext;
  modalState: ModalState;

  setActiveTrack: (trackId: number) => void;
  setActiveRegionSet: (regionSetId: number) => void;
  setActiveRegion: (regionId: number) => void;
  setActiveGraph: (graphId: number) => void;
  clearActiveSelection: () => void;

  copyToClipboard: (clipboard: Clipboard) => void;
  clearClipboard: () => void;

  openContextMenu: (context: RightClickContext) => void;
  closeContextMenu: () => void;

  openModal: (modal: ModalState) => void;
  closeModal: () => void;

  closeAllUI: () => void;
};

const EMPTY_SELECTION: ActiveSelection = { trackId: null, regionSetId: null, regionId: null, graphId: null };

export const useUIStore: UseBoundStore<StoreApi<UIStore>> = create<UIStore>()(
  subscribeWithSelector(
    devtools<UIStore>(
      (set) => ({
        activeSelection: EMPTY_SELECTION,
        clipboard: null,
        rightClickContext: null,
        modalState: null,

        setActiveTrack: (trackId) =>
          set({ activeSelection: { trackId, regionSetId: null, regionId: null, graphId: null } }, false, 'setActiveTrack'),

        setActiveRegionSet: (regionSetId) => {
          const regionSet = useRegionSetStore.getState().getRegionSet(regionSetId);
          if (!regionSet) return;
          set({ activeSelection: { trackId: regionSet.trackId, regionSetId, regionId: null, graphId: null } }, false, 'setActiveRegionSet');
        },

        setActiveRegion: (regionId) => {
          const region = useRegionStore.getState().getRegion(regionId);
          if (!region) return;
          const regionSet = useRegionSetStore.getState().getRegionSet(region.regionSetId);
          if (!regionSet) return;
          set({ activeSelection: { trackId: regionSet.trackId, regionSetId: region.regionSetId, regionId, graphId: null } }, false, 'setActiveRegion');
        },

        setActiveGraph: (graphId) => {
          const graph = useGraphStore.getState().getGraph(graphId);
          if (!graph || !graph.regionId) return;
          const region = useRegionStore.getState().getRegion(graph.regionId);
          if (!region) return;
          const regionSet = useRegionSetStore.getState().getRegionSet(region.regionSetId);
          if (!regionSet) return;
          set({ activeSelection: { trackId: regionSet.trackId, regionSetId: region.regionSetId, regionId: graph.regionId, graphId } }, false, 'setActiveGraph');
        },

        clearActiveSelection: () =>
          set({ activeSelection: EMPTY_SELECTION }, false, 'clearActiveSelection'),

        copyToClipboard: (clipboard) =>
          set({ clipboard }, false, 'copyToClipboard'),

        clearClipboard: () =>
          set({ clipboard: null }, false, 'clearClipboard'),

        openContextMenu: (context) =>
          set({ rightClickContext: context }, false, 'openContextMenu'),

        closeContextMenu: () =>
          set({ rightClickContext: null }, false, 'closeContextMenu'),

        openModal: (modal) =>
          set({ modalState: modal }, false, 'openModal'),

        closeModal: () =>
          set({ modalState: null }, false, 'closeModal'),

        closeAllUI: () =>
          set({ modalState: null, rightClickContext: null }, false, 'closeAllUI'),
      }),
      { name: 'UIStore' }
    )
  )
);
