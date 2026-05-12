import { useEffect, useCallback } from 'react';
import { useStreamPlayer } from '@/Audio/AudioEngineContext';
import { useActiveGraphState } from '@/Stores/ActiveGraphState';
import { useWasmBinaryStore } from '@/Stores/WasmBinaryStore';
import { compileActiveGraph } from '@/audio/pipeline/GraphCompiler';
import type { AudioCommand } from '@/audio/transport/AudioCommandDispatcher';

export function useAudioEngine() {
  const player      = useStreamPlayer();
  const activeGraph = useActiveGraphState((s) => s.activeGraph);
  const binaries    = useWasmBinaryStore((s) => s.binaries);

  useEffect(() => {
    if (!activeGraph) return;

    player.dispatch({ type: 'SET_BYPASS', bypass: !activeGraph.enabled });

    if (!activeGraph.enabled) return;

    const descriptor = compileActiveGraph(activeGraph, binaries);
    if (!descriptor) return;

    player.dispatch({
      type:          'SET_GRAPH',
      compiledGraph: descriptor.compiledGraph,
      binaries:      descriptor.binaries,
    });
  }, [activeGraph, binaries, player]);

  const dispatch = useCallback(
    (command: AudioCommand) => player.dispatch(command),
    [player],
  );

  return { dispatch };
}
