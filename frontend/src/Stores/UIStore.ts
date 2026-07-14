import { create, type StoreApi, type UseBoundStore } from 'zustand';
import { devtools, subscribeWithSelector } from 'zustand/middleware';
import type { PasteGraphParams, PasteRegionParams, PasteRegionSetParams } from './PasteParams';
import { useRegionSetStore } from './RegionSetStore';
import { useRegionStore } from './RegionStore';
import { useGraphStore } from './GraphStore';
import type { CanonicalAudio } from '@/audio/canonicalAudio';

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

export type EditingRegionBounds = null | {
  regionId: number;
  regionSetId: number;
  originalStart: number;
  originalEnd: number;
  start: number;
  end: number;
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
  | { type: 'unsavedRegionBounds'; nextSelection: ActiveSelection }
  // Graph modals
  | { type: 'createGraph'; regionId: number }
  | { type: 'renameGraph'; graphId: number }
  | { type: 'detailsGraph'; graphId: number }
  | { type: 'deleteGraph'; graphId: number }
  | { type: 'pasteGraph'; params: PasteGraphParams }
  // Canvas node modals
  | { type: 'nodeDetails'; nodeId: number | null; transformId: number | null }
  | { type: 'clearGraphNodes'; graphId: number }
  // Transform store modals
  | { type: 'createTransform' }
  | { type: 'transformDetails'; transformId: number }
  | null;

export type UIStore = {
  activeSelection: ActiveSelection;
  editingRegionBounds: EditingRegionBounds;
  clipboard: Clipboard;
  rightClickContext: RightClickContext;
  modalState: ModalState;

  setActiveTrack: (trackId: number) => void;
  setActiveRegionSet: (regionSetId: number) => void;
  setActiveRegion: (regionId: number) => void;
  setActiveGraph: (graphId: number) => void;
  clearActiveRegion: () => void;
  clearActiveSelection: () => void;
  applyActiveSelection: (selection: ActiveSelection) => void;

  beginEditingRegionBounds: (regionId: number) => void;
  updateEditingRegionBounds: (start: number, end: number) => void;
  clearEditingRegionBounds: () => void;

  copyToClipboard: (clipboard: Clipboard) => void;
  clearClipboard: () => void;

  openContextMenu: (context: RightClickContext) => void;
  closeContextMenu: () => void;

  openModal: (modal: ModalState) => void;
  closeModal: () => void;

  closeAllUI: () => void;
};

const EMPTY_SELECTION: ActiveSelection = { trackId: null, regionSetId: null, regionId: null, graphId: null };
const sameSelection = (left: ActiveSelection, right: ActiveSelection) =>
  left.trackId === right.trackId &&
  left.regionSetId === right.regionSetId &&
  left.regionId === right.regionId &&
  left.graphId === right.graphId;

const isEditingRegionDirty = (editing: EditingRegionBounds) =>
  editing != null && (editing.start !== editing.originalStart || editing.end !== editing.originalEnd);

const isLeavingEditedRegion = (editing: EditingRegionBounds, nextSelection: ActiveSelection) =>
  editing != null && nextSelection.regionId !== editing.regionId;

export const useUIStore: UseBoundStore<StoreApi<UIStore>> = create<UIStore>()(
  subscribeWithSelector(
    devtools<UIStore>(
      (set, get) => {
        const requestSelectionChange = (nextSelection: ActiveSelection, actionName: string) => {
          const currentSelection = get().activeSelection;
          if (sameSelection(currentSelection, nextSelection)) {
            return;
          }

          const editing = get().editingRegionBounds;
          if (!isLeavingEditedRegion(editing, nextSelection)) {
            set({ activeSelection: nextSelection }, false, actionName);
            return;
          }

          if (isEditingRegionDirty(editing)) {
            set({ modalState: { type: 'unsavedRegionBounds', nextSelection } }, false, `${actionName}:promptUnsavedRegionBounds`);
            return;
          }

          set({ activeSelection: nextSelection, editingRegionBounds: null }, false, actionName);
        };

        return {
          activeSelection: EMPTY_SELECTION,
          editingRegionBounds: null,
          clipboard: null,
          rightClickContext: null,
          modalState: null,

          setActiveTrack: (trackId) =>
            requestSelectionChange({ trackId, regionSetId: null, regionId: null, graphId: null }, 'setActiveTrack'),

          setActiveRegionSet: (regionSetId) => {
            const regionSet = useRegionSetStore.getState().getRegionSet(regionSetId);
            if (!regionSet) return;
            requestSelectionChange({ trackId: regionSet.trackId, regionSetId, regionId: null, graphId: null }, 'setActiveRegionSet');
          },

          setActiveRegion: (regionId) => {
            const region = useRegionStore.getState().getRegion(regionId);
            if (!region) return;
            const regionSet = useRegionSetStore.getState().getRegionSet(region.regionSetId);
            if (!regionSet) return;
            requestSelectionChange({ trackId: regionSet.trackId, regionSetId: region.regionSetId, regionId, graphId: null }, 'setActiveRegion');
          },

          setActiveGraph: (graphId) => {
            const graph = useGraphStore.getState().getGraph(graphId);
            if (!graph || !graph.regionId) return;
            const region = useRegionStore.getState().getRegion(graph.regionId);
            if (!region) return;
            const regionSet = useRegionSetStore.getState().getRegionSet(region.regionSetId);
            if (!regionSet) return;
            requestSelectionChange({ trackId: regionSet.trackId, regionSetId: region.regionSetId, regionId: graph.regionId, graphId }, 'setActiveGraph');
          },

          clearActiveRegion: () => {
            const currentSelection = get().activeSelection;
            requestSelectionChange(
              {
                trackId: currentSelection.trackId,
                regionSetId: currentSelection.regionSetId,
                regionId: null,
                graphId: null,
              },
              'clearActiveRegion'
            );
          },

          clearActiveSelection: () =>
            requestSelectionChange(EMPTY_SELECTION, 'clearActiveSelection'),

          applyActiveSelection: (selection) =>
            set({ activeSelection: selection }, false, 'applyActiveSelection'),

          beginEditingRegionBounds: (regionId) => {
            const region = useRegionStore.getState().getRegion(regionId);
            if (!region) return;
            const regionSet = useRegionSetStore.getState().getRegionSet(region.regionSetId);
            if (!regionSet) return;

            set({
              activeSelection: {
                trackId: regionSet.trackId,
                regionSetId: region.regionSetId,
                regionId,
                graphId: null,
              },
              editingRegionBounds: {
                regionId,
                regionSetId: region.regionSetId,
                originalStart: region.start,
                originalEnd: region.end,
                start: region.start,
                end: region.end,
              },
            }, false, 'beginEditingRegionBounds');
          },

          updateEditingRegionBounds: (start, end) =>
            set((state) => {
              if (!state.editingRegionBounds) return {};
              return {
                editingRegionBounds: {
                  ...state.editingRegionBounds,
                  start,
                  end,
                },
              };
            }, false, 'updateEditingRegionBounds'),

          clearEditingRegionBounds: () =>
            set({ editingRegionBounds: null }, false, 'clearEditingRegionBounds'),

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
        };
      },
      { name: 'UIStore' }
    )
  )
);
