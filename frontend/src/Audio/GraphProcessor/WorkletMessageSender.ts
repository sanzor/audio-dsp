/**
 * WorkletMessageSender — main-thread side of the worklet message channel.
 *
 * Wraps the AudioWorkletNode's MessagePort with typed send methods and
 * typed response subscriptions.  Nothing in the rest of the codebase
 * should call port.postMessage directly.
 *
 * Sending:
 *   sendGraph()        → SET_GRAPH  (binaries are sliced + transferred)
 *   sendBypass()       → SET_BYPASS
 *   sendUpdateParams() → UPDATE_PARAMS
 *
 * Receiving (worklet → main thread):
 *   onGraphReady()     → GRAPH_READY
 *   onModuleError()    → MODULE_ERROR
 *   Both return an unsubscribe function.
 */

import type { CompiledGraph } from './dto/CompiledGraph';
import type { WorkletInboundMessage, WorkletOutboundMessage } from './dto/WorkletMessage';

type UnsubscribeFn = () => void;
type HandlerSet     = Set<(data: WorkletOutboundMessage) => void>;

export class WorkletMessageSender {
  private handlers = new Map<string, HandlerSet>();

  constructor(private port: MessagePort) {
    port.onmessage = ({ data }: MessageEvent<WorkletOutboundMessage>) => {
      this.handlers.get(data.type)?.forEach((cb) => cb(data));
    };
  }

  // ── Outbound (main → worklet) ──────────────────────────────────────────────

  sendGraph(graph: CompiledGraph, binaries: Record<number, Uint8Array>): void {
    const transferableBinaries: Record<number, ArrayBuffer> = {};
    const transfers: ArrayBuffer[] = [];

    for (const [id, bin] of Object.entries(binaries)) {
      const copy = bin.slice().buffer; // copy so WasmBinaryStore keeps its original
      transferableBinaries[Number(id)] = copy;
      transfers.push(copy);
    }

    const message: WorkletInboundMessage = {
      type: 'SET_GRAPH',
      graph,
      binaries: transferableBinaries,
    };
    this.port.postMessage(message, transfers);
  }

  sendBypass(bypass: boolean): void {
    const message: WorkletInboundMessage = { type: 'SET_BYPASS', bypass };
    this.port.postMessage(message);
  }

  sendUpdateParams(nodeIndex: number, params: number[]): void {
    const message: WorkletInboundMessage = { type: 'UPDATE_PARAMS', nodeIndex, params };
    this.port.postMessage(message);
  }

  // ── Inbound (worklet → main) ───────────────────────────────────────────────

  onGraphReady(cb: () => void): UnsubscribeFn {
    return this.subscribe('GRAPH_READY', () => cb());
  }

  onModuleError(cb: (transformId: number, error: string) => void): UnsubscribeFn {
    return this.subscribe('MODULE_ERROR', (data) => {
      if (data.type === 'MODULE_ERROR') cb(data.transformId, data.error);
    });
  }

  // ── Private ────────────────────────────────────────────────────────────────

  private subscribe(
    type: string,
    cb: (data: WorkletOutboundMessage) => void,
  ): UnsubscribeFn {
    if (!this.handlers.has(type)) this.handlers.set(type, new Set());
    this.handlers.get(type)!.add(cb);
    return () => this.handlers.get(type)?.delete(cb);
  }
}
