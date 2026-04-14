/**
 * useAudioEngine
 *
 * Wires the active graph + binary store to the StreamPlayer.
 * Recompiles and dispatches SET_GRAPH whenever the graph topology,
 * params, or available binaries change.
 *
 * Returns a single dispatch function — the only way components
 * should interact with the audio system:
 *
 *   const { dispatch } = useAudioEngine()
 *
 *   // play with effects
 *   dispatch({ type: 'SET_BYPASS', bypass: false })
 *   dispatch({ type: 'PLAY', audioBuffer })
 *
 *   // play raw (bypass graph)
 *   dispatch({ type: 'SET_BYPASS', bypass: true })
 *   dispatch({ type: 'PLAY', audioBuffer })
 */

import { useEffect, useRef, useCallback } from 'react';
import { useStreamPlayer } from '@/Audio/AudioEngineContext';
import { useActiveGraphState } from '@/Stores/ActiveGraphState';
import { useWasmBinaryStore } from '@/Stores/WasmBinaryStore';
import { GraphTransformerProvider } from '@/Audio/GraphProcessor/GraphTransformerProvider';
import type { AudioCommand } from '@/Audio/GraphProcessor/dto/AudioCommand';

export function useAudioEngine() {
  const player      = useStreamPlayer();
  const activeGraph = useActiveGraphState((s) => s.activeGraph);
  const getBinary   = useWasmBinaryStore((s) => s.getBinary);
  const binaries    = useWasmBinaryStore((s) => s.binaries); // tracked as dep so effect re-runs when any binary loads

  const providerRef = useRef<GraphTransformerProvider | null>(null);

  useEffect(() => {
    if (!activeGraph) return;

    // Sync bypass with graph.enabled.
    player.dispatch({ type: 'SET_BYPASS', bypass: !activeGraph.enabled });

    if (!activeGraph.enabled) return;

    // Create or update the provider.
    if (!providerRef.current) {
      providerRef.current = new GraphTransformerProvider(activeGraph, getBinary);
    } else {
      providerRef.current.update(activeGraph);
    }

    const descriptor = providerRef.current.getDescriptor();
    if (!descriptor) return; // binaries not yet all loaded — effect will re-run when they arrive

    player.dispatch({
      type:          'SET_GRAPH',
      compiledGraph: descriptor.compiledGraph,
      binaries:      descriptor.binaries,
    });
  }, [activeGraph, binaries, getBinary, player]);

  const dispatch = useCallback(
    (command: AudioCommand) => player.dispatch(command),
    [player],
  );

  return { dispatch };
}
