import { create } from "zustand";

interface AudioCacheEntry {
  blob: Blob;
  objectUrl: string;
}

interface AudioCacheStore {
  cache: Map<number, AudioCacheEntry>;
  hasAudio: (trackId: number) => boolean;
  getUrl: (trackId: number) => string | undefined;
  setAudio: (trackId: number, blob: Blob) => void;
}

export const useAudioCacheStore = create<AudioCacheStore>((set, get) => ({
  cache: new Map(),

  hasAudio: (trackId) => get().cache.has(trackId),

  getUrl: (trackId) => get().cache.get(trackId)?.objectUrl,

  setAudio: (trackId, blob) => {
    const existing = get().cache.get(trackId);
    if (existing) return; // already cached, do nothing

    const objectUrl = URL.createObjectURL(blob);
    set((state) => {
      const next = new Map(state.cache);
      next.set(trackId, { blob, objectUrl });
      return { cache: next };
    });
  },
}));
