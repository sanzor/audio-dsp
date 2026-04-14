/**
 * WorkletMessage — messages that cross the thread boundary.
 *
 * Inbound  = main thread → worklet   (commands that affect DSP processing)
 * Outbound = worklet → main thread   (status responses)
 *
 * ArrayBuffers in SET_GRAPH are transferred (zero-copy ownership move),
 * everything else is structured-cloned.
 */

import type { CompiledGraph } from './CompiledGraph';

export type WorkletInboundMessage =
  | { type: 'SET_GRAPH';     graph: CompiledGraph; binaries: Record<number, ArrayBuffer> }
  | { type: 'SET_BYPASS';    bypass: boolean }
  | { type: 'UPDATE_PARAMS'; nodeIndex: number; params: number[] };

export type WorkletOutboundMessage =
  | { type: 'GRAPH_READY' }
  | { type: 'MODULE_ERROR'; transformId: number; error: string };
