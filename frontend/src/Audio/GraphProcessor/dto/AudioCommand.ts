/**
 * AudioCommand — every command the audio system can receive.
 *
 * PLAY / STOP / PAUSE  → handled on the main thread by StreamPlayer
 *                         (controls the AudioBufferSourceNode, never touches the worklet)
 *
 * SET_GRAPH            → compiled graph + binaries sent to the worklet
 * SET_BYPASS           → toggle graph effects on/off in the worklet
 * UPDATE_PARAMS        → update a node's params without recompiling the graph
 */

import type { CompiledGraph } from './CompiledGraph';

export type AudioCommand =
  | { type: 'PLAY';          audioBuffer: AudioBuffer }
  | { type: 'STOP' }
  | { type: 'PAUSE' }
  | { type: 'SET_GRAPH';     compiledGraph: CompiledGraph; binaries: Record<number, Uint8Array> }
  | { type: 'SET_BYPASS';    bypass: boolean }
  | { type: 'UPDATE_PARAMS'; nodeIndex: number; params: number[] };
