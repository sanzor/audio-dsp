import type { CompiledGraph } from '@/audio/pipeline/compile-graph/compiledGraph';

// ─── Message types ────────────────────────────────────────────────────────────

export type WorkletInboundMessage =
  | { type: 'SET_GRAPH';     graph: CompiledGraph; binaries: Record<number, ArrayBuffer> }
  | { type: 'SET_BYPASS';    bypass: boolean }
  | { type: 'UPDATE_PARAMS'; nodeIndex: number; params: number[] };

export type WorkletOutboundMessage =
  | { type: 'GRAPH_READY' }
  | { type: 'MODULE_ERROR'; transformId: number; error: string };

// ─── Sender ───────────────────────────────────────────────────────────────────

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
      const copy = bin.slice().buffer;
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

  dispose(): void {
    this.port.onmessage = null
    this.handlers.clear()
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
