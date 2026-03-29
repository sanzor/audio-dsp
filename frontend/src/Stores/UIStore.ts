// stores/uiStore.ts
import { create, type StoreApi, type UseBoundStore } from 'zustand';
import { devtools, subscribeWithSelector } from 'zustand/middleware';
import type { PasteGraphParams, PasteRegionParams, PasteRegionSetParams } from './PasteParams';
import type { CanonicalAudio } from '@/Audio/CanonicalAudio';

// Your existing types
export type RightClickContext =
  | { type: 'track'; trackId: number; x: number; y: number }
  | { type: 'region'; regionId: number; x: number; y: number }
  | { type: 'regionSet'; regionSetId: number; x: number; y: number }
  | { type: 'graph'; graphId: number; x: number; y: number }
  | null;

export type SelectedContext =
  | { type: 'track'; trackId: number }
  | { type: 'regionSet'; regionSetId: number }
  | { type: 'region'; regionId: number }
  | { type: 'graph'; graphId: number }
  | null;

export type OpenedContext =
  | { type: 'track'; trackId: number }
  | { type: 'regionSet'; regionSetId: number }
  | { type: 'region'; regionId: number }
  | { type: 'graph'; graphId: number }
  | null;

export type Clipboard =
  | { type: 'track'; trackId: number }
  | { type: 'regionSet'; regionSetId: number }
  | { type: 'region'; regionId: number }
  | { type: 'graph'; graphId: number }
  | { type: 'node'; nodeId: number }
  | { type: 'edge'; edgeId: number }
  | null;

// Modal state for ALL your modals
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
  
  // ... add more as needed
  | null;


export type UIStore = {
  // State
  selectedContext: SelectedContext;
  clipboard: Clipboard;
  openedContext: OpenedContext;
  rightClickContext: RightClickContext;
  modalState: ModalState;
  
  // Actions
  select: (context: SelectedContext) => void;
  clearSelection: () => void;

  copyToClipboard: (clipboard: Clipboard) => void;
  clearClipboard:()=>void;
  
  open: (context: OpenedContext) => void;
  close:()=>void;

  openContextMenu: (context: RightClickContext) => void;
  closeContextMenu: () => void;

  openModal: (modal: ModalState) => void;
  closeModal: () => void;
  
  // Composite actions
  closeAllUI: () => void;
};

export const useUIStore: UseBoundStore<StoreApi<UIStore>> = create<UIStore>()(
  subscribeWithSelector(
    devtools<UIStore>(
      (set) => ({
        // Initial state
        selectedContext: null,
        clipboard: null,
        openedContext: null,
        rightClickContext: null,
        modalState: null,
        
        // Actions
        select: (context: SelectedContext) =>
          set({ selectedContext: context }, false, 'setSelectedContext'),
        clearSelection: () => 
          set({ selectedContext: null }, false, 'clearSelection'),
        
        copyToClipboard: (clipboard: Clipboard) =>
          set({ clipboard }, false, 'copyToClipboard'),
        clearClipboard: () => set({ clipboard: null }, false, 'clearClipboard'),

        
        open: (context: OpenedContext) =>
          set({ openedContext: context }, false, 'open'),
        close: () => set({ openedContext: null }, false, 'close'),

        
        openContextMenu: (context: RightClickContext) =>
          set({ rightClickContext: context }, false, 'openContextMenu'),

        closeContextMenu: () => set({ rightClickContext: null }, false, 'closeContextMenu'),
        
        openModal: (modal: ModalState) =>
          set({ modalState: modal }, false, 'openModal'),
        
        closeModal: () => 
          set({ modalState: null }, false, 'closeModal'),
        
        // Composite actions
       
        
       
        closeAllUI: () => 
          set({ 
            modalState: null, 
            rightClickContext: null 
          }, false, 'closeAllUI'),
      }),
      { name: 'UIStore' }
    )
  )
);
