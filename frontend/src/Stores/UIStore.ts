// stores/uiStore.ts
import { create, type StoreApi, type UseBoundStore } from 'zustand';
import { devtools, subscribeWithSelector } from 'zustand/middleware';
import type { PasteGraphParams, PasteRegionParams, PasteRegionSetParams } from './PasteParams';
import type { CanonicalAudio } from '@/Audio/CanonicalAudio';

// Your existing types
export type RightClickContext =
  | { type: 'track'; trackId: string; x: number; y: number }
  | { type: 'region'; regionId: string; x: number; y: number }
  | { type: 'regionSet'; regionSetId: string; x: number; y: number }
  | { type: 'graph'; graphId: string; x: number; y: number }
  | { type: 'waveform_region'; regionId: string; x: number; y: number } // Right-click on an existing region graphic
  | { type: 'waveform_timeline'; regionSetId: string; time: number; x: number; y: number } // Right-click on the timeline background
  | null;

export type SelectedContext =
  | { type: 'track'; trackId: string }
  | { type: 'regionSet'; regionSetId: string }
  | { type: 'region'; regionId: string }
  | { type: 'graph'; graphId: string }
  | null;

export type OpenedContext =
  | { type: 'track'; trackId: string }
  | { type: 'regionSet'; regionSetId: string }
  | { type: 'region'; regionId: string }
  | { type: 'graph'; graphId: string }
  | null;

export type Clipboard =
  | { type: 'track'; trackId: string }
  | { type: 'regionSet'; regionSetId: string }
  | { type: 'region'; regionId: string }
  | { type: 'graph'; graphId: string }
  | { type: 'node'; nodeId: string }
  | { type: 'edge'; edgeId: string }
  | null;

// Modal state for ALL your modals
export type ModalState = 
  // Track modals
  | { type: 'createTrack'; canonicalAudio: CanonicalAudio | null }
  | { type: 'renameTrack'; trackId: string }
  | { type: 'detailsTrack'; trackId: string }
  | { type: 'deleteTrack'; trackId: string }
  | { type: 'pasteTrack'; trackId: string }
  
  // RegionSet modals
  | { type: 'createRegionSet'; trackId: string }
  | { type: 'renameRegionSet'; regionSetId: string }
  | { type: 'detailsRegionSet'; regionSetId: string }
  | { type: 'deleteRegionSet'; regionSetId: string }
  | { type: 'pasteRegionSet'; params: PasteRegionSetParams }
  
  // Region modals
  | { type: 'createRegion'; regionSetId: string; startTime?: number; endTime?: number }
  | { type: 'renameRegion'; regionId: string }
  | { type: 'detailsRegion'; regionId: string }
  | { type: 'pasteRegion'; params: PasteRegionParams }
  | { type: 'deleteRegion'; regionId: string }
  
  // Graph modals
  | { type: 'createGraph'; regionId: string }
  | { type: 'renameGraph'; graphId: string }
  | { type: 'detailsGraph'; graphId: string }
  | { type: 'deleteGraph'; graphId: string }
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
