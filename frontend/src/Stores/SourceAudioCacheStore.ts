import { create } from "zustand";

// Mirrors AudioCacheStore.ts's shape exactly, but keyed by source_id instead
// of track_id -- kept as a separate store (not a shared cache) since the two
// id spaces are unrelated and could otherwise collide.

interface SourceAudioCacheEntry {
  blob: Blob;
  objectUrl: string;
}

interface SourceAudioCacheStore {
  cache: Map<number, SourceAudioCacheEntry>;
  hasAudio: (sourceId: number) => boolean;
  getUrl: (sourceId: number) => string | undefined;
  setAudio: (sourceId: number, blob: Blob) => void;
}

export const useSourceAudioCacheStore = create<SourceAudioCacheStore>((set, get) => ({
  cache: new Map(),

  hasAudio: (sourceId) => get().cache.has(sourceId),

  getUrl: (sourceId) => get().cache.get(sourceId)?.objectUrl,

  setAudio: (sourceId, blob) => {
    const existing = get().cache.get(sourceId);
    if (existing) return; // already cached, do nothing

    const objectUrl = URL.createObjectURL(blob);
    set((state) => {
      const next = new Map(state.cache);
      next.set(sourceId, { blob, objectUrl });
      return { cache: next };
    });
  },
}));
