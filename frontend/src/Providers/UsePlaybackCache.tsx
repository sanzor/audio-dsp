import { useContext } from "react";
import { AudioPlaybackCacheContext } from "./AudioPlaybackCacheContext";

export const useAudioPlaybackCache = () => {
  const context = useContext(AudioPlaybackCacheContext);
  if (!context) {
    throw new Error("useAudioPlaybackCache must be used within an AudioPlaybackCacheProvider");
  }
  return context;
};