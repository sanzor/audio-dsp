import type { CompiledGraph } from '@/audio/pipeline/compile-graph/compiledGraph';

// ─── Message types ────────────────────────────────────────────────────────────

export type WorkletInboundMessage =
  | { type: 'SET_GRAPH';     graph: CompiledGraph; binaries: Record<number, ArrayBuffer> }
  | { type: 'SET_BYPASS';    bypass: boolean }
  | { type: 'UPDATE_PARAMS'; nodeIndex: number; params: number[] };

export type WorkletOutboundMessage =
  | { type: 'GRAPH_READY' }
  | { type: 'MODULE_ERROR'; transformId: number; error: string }
  | { type: 'RUNTIME_ERROR'; nodeIndex: number; transformId: number | null; error: string }
  | { type: 'CPU_LOAD'; value: number };

// A transform trapped inside the worklet's process(). The worklet has already
// retired the graph (dry pass-through) and stays that way until the next
// SET_GRAPH. `nodeIndex` is the position in CompiledGraph.executionOrder, or
// -1 if the failure couldn't be attributed to a node.
export interface WorkletRuntimeError {
  nodeIndex: number;
  transformId: number | null;
  error: string;
}

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

  onRuntimeError(cb: (error: WorkletRuntimeError) => void): UnsubscribeFn {
    return this.subscribe('RUNTIME_ERROR', (data) => {
      if (data.type === 'RUNTIME_ERROR') {
        cb({ nodeIndex: data.nodeIndex, transformId: data.transformId, error: data.error });
      }
    });
  }

  onCpuLoad(cb: (value: number) => void): UnsubscribeFn {
    return this.subscribe('CPU_LOAD', (data) => {
      if (data.type === 'CPU_LOAD') cb(data.value);
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
