import { createContext, useCallback, useState, type ReactNode } from "react";

interface AudioPlaybackCacheContextProps{
    getBlob:(trackId:string)=>Blob|undefined;
    setBlob:(trackId:string,blob:Blob)=>void;
    hasBlob:(trackId:string)=>boolean;
}

// eslint-disable-next-line react-refresh/only-export-components
export const AudioPlaybackCacheContext=createContext<AudioPlaybackCacheContextProps|null>(null);

export const AudioPlaybackCacheProvider = ({ children }: { children: ReactNode }) => {
  const [audioCache, setAudioCache] = useState<Map<string, Blob>>(new Map());

  const getBlob = useCallback((trackId: string): Blob | undefined => {
    return audioCache.get(trackId);
  }, [audioCache]);

  const setBlob = useCallback((trackId: string, blob: Blob) => {
    setAudioCache(prev => {
      const newCache = new Map(prev);
      newCache.set(trackId, blob);
      return newCache;
    });
  }, []);

  const hasBlob = useCallback((trackId: string): boolean => {
    return audioCache.has(trackId);
  }, [audioCache]);

  return (
    <AudioPlaybackCacheContext.Provider value={{ getBlob, setBlob, hasBlob }}>
      {children}
    </AudioPlaybackCacheContext.Provider>
  );
};