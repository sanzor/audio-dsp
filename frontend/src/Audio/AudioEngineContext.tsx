/**
 * AudioEngineContext
 *
 * Provides a single StreamPlayer instance to the component tree.
 * Created once when the Provider mounts, destroyed when it unmounts.
 *
 * Place <AudioEngineProvider> at the dashboard root (inside the auth
 * boundary, so it only exists while a user is logged in).
 *
 * Usage in components:
 *   const { dispatch } = useAudioEngine()
 *   dispatch({ type: 'PLAY', audioBuffer })
 *   dispatch({ type: 'SET_BYPASS', bypass: true })
 */

import { createContext, useContext, useRef, useEffect, type ReactNode } from 'react';
import { StreamPlayer } from './StreamPlayer';

const AudioEngineContext = createContext<StreamPlayer | null>(null);

export function AudioEngineProvider({ children }: { children: ReactNode }) {
  const playerRef = useRef<StreamPlayer>(new StreamPlayer());

  useEffect(() => {
    return () => playerRef.current.destroy();
  }, []);

  return (
    <AudioEngineContext.Provider value={playerRef.current}>
      {children}
    </AudioEngineContext.Provider>
  );
}

export function useStreamPlayer(): StreamPlayer {
  const player = useContext(AudioEngineContext);
  if (!player) throw new Error('useStreamPlayer must be used inside <AudioEngineProvider>');
  return player;
}
