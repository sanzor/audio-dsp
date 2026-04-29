import { create } from 'zustand';

type BinaryStatus = 'fetching' | 'ready' | 'error';

interface WasmBinaryState {
  binaries: Map<number, Uint8Array>;
  status: Map<number, BinaryStatus>;
  setBinary(transformId: number, binary: Uint8Array): void;
  setStatus(transformId: number, status: 'fetching' | 'error'): void;
  getBinary(transformId: number): Uint8Array | null;
  isReady(transformId: number): boolean;
  clear(): void;
}

export const useWasmBinaryStore = create<WasmBinaryState>()((set, get) => ({
  binaries: new Map(),
  status: new Map(),

  setBinary(transformId, binary) {
    set((s) => {
      const binaries = new Map(s.binaries);
      binaries.set(transformId, binary);
      const status = new Map(s.status);
      status.set(transformId, 'ready');
      return { binaries, status };
    });
  },

  setStatus(transformId, st) {
    set((s) => {
      const status = new Map(s.status);
      status.set(transformId, st);
      return { status };
    });
  },

  getBinary(transformId) {
    return get().binaries.get(transformId) ?? null;
  },

  isReady(transformId) {
    return get().status.get(transformId) === 'ready';
  },

  clear() {
    set({ binaries: new Map(), status: new Map() });
  },
}));
