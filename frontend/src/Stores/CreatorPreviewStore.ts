import { create } from "zustand";
import { CreatorTransformPreview } from "@/components/creator/creatorTransformPreview";
import type { CompiledGraph } from "@/audio/pipeline/compile-graph/compiledGraph";

// Shared "Try it" live-audition session for the creator surface. Unpersisted
// — mirrors WorkletStore.ts's shape (a module-scope class instance held
// directly as state, no `persist` middleware) rather than CreatorStore.ts's
// persisted store, since none of this is meant to survive a reload.
//
// Raw Web Audio objects (oscillator, analysers, the rAF handle, the CPU-load
// unsubscribe fn) are module-scope `let`s, not store state — only the derived
// numbers components actually render (levels, cpu%, latency) go through
// Zustand, so subscribers re-render on data changes, not on node churn.

export type PreviewStatus = "idle" | "loading" | "playing" | "error";

export interface LevelReading {
  peak: number;
  rms: number;
}

const SILENT_LEVEL: LevelReading = { peak: 0, rms: 0 };

// Throttles both the meter-reading rate and (independently, worklet-side)
// the CPU_LOAD post rate — an un-throttled rAF loop driving Zustand `set()`
// at 60fps would force every subscriber to re-render 60 times/sec.
const METER_UPDATE_INTERVAL_MS = 1000 / 20;

const TEST_TONE_FREQUENCY_HZ = 440;
const RENDER_QUANTUM_SIZE = 128;

const preview = new CreatorTransformPreview();

let oscillator: OscillatorNode | null = null;
let inputAnalyser: AnalyserNode | null = null;
let outputAnalyser: AnalyserNode | null = null;
let inputBuffer: Float32Array | null = null;
let outputBuffer: Float32Array | null = null;
let rafHandle: number | null = null;
let lastMeterUpdate = 0;
let unsubscribeCpuLoad: (() => void) | null = null;

interface CreatorPreviewState {
  previewTransformId: number | null;
  // Opaque staleness key each caller builds itself — primitives use
  // `${resourceId}:${sourceCode}`, composites a stringified hash of the
  // current graph state. Keeps this store agnostic to which kind of
  // transform it's previewing.
  previewResourceKey: string | null;
  status: PreviewStatus;
  error: string | null;
  bypassed: boolean;
  paramValues: number[];
  // The exact CompiledGraph currently loaded into the worklet — kept around
  // (not just consumed and discarded at play() time) so composite-canvas.tsx
  // can look up a node's live executionOrder index by node_id on demand, at
  // the moment of each param edit. Null whenever nothing is previewing.
  previewGraph: CompiledGraph | null;
  // Per-node live param values for the currently loaded preview graph, keyed
  // by node_id (== CompiledNode.nodeId — for the single-node primitive
  // preview graph that's PRIMITIVE_PREVIEW_NODE_ID). Separate from the flat
  // `paramValues` above, which is the primitive "Try it" path's own
  // single-transform param state and is left untouched by this addition.
  nodeParamValues: Map<number, number[]>;
  inputLevel: LevelReading;
  outputLevel: LevelReading;
  cpuLoadPct: number | null;
  latencyMs: number | null;
  play: (
    transformId: number,
    resourceKey: string,
    graph: CompiledGraph,
    binaries: Record<number, Uint8Array>,
    params: number[]
  ) => Promise<void>;
  stop: () => void;
  setBypass: (bypass: boolean) => void;
  updateParam: (index: number, value: number) => void;
  // Ephemeral per-node param edit for the composite canvas's Details tab
  // (agents/decisions/0005-composite-node-inspector.md, Phase 2). Resolves
  // `nodeId` to its current executionOrder index against `previewGraph`
  // fresh on every call — deliberately not cached/memoized, since Phase 3's
  // disabled-node filtering changes which nodes are even present in the
  // compiled graph between compiles. No-ops if the node isn't in the
  // currently loaded preview graph (not previewing, or filtered out as
  // disabled). Never written into editingGraph, never part of Save/Publish.
  updateNodeParam: (nodeId: number, paramIndex: number, value: number) => void;
}

function readLevel(analyser: AnalyserNode, buffer: Float32Array): LevelReading {
  analyser.getFloatTimeDomainData(buffer);
  let peak = 0;
  let sumSquares = 0;
  for (let i = 0; i < buffer.length; i++) {
    const abs = Math.abs(buffer[i]);
    if (abs > peak) peak = abs;
    sumSquares += buffer[i] * buffer[i];
  }
  return { peak, rms: Math.sqrt(sumSquares / buffer.length) };
}

// Tears down everything from a prior session — safe to call when nothing is
// running. Always runs before starting a new session (rapid transform
// switches, double-clicking Play) and on explicit Stop.
function teardownSession(): void {
  if (rafHandle != null) {
    cancelAnimationFrame(rafHandle);
    rafHandle = null;
  }
  unsubscribeCpuLoad?.();
  unsubscribeCpuLoad = null;

  try {
    oscillator?.stop();
  } catch {
    // Already stopped or never started — fine either way.
  }
  oscillator?.disconnect();
  oscillator = null;
  inputAnalyser?.disconnect();
  inputAnalyser = null;
  outputAnalyser?.disconnect();
  outputAnalyser = null;
  inputBuffer = null;
  outputBuffer = null;

  preview.dispose();
}

export const useCreatorPreviewStore = create<CreatorPreviewState>()((set, get) => ({
  previewTransformId: null,
  previewResourceKey: null,
  status: "idle",
  error: null,
  bypassed: false,
  paramValues: [],
  previewGraph: null,
  nodeParamValues: new Map(),
  inputLevel: SILENT_LEVEL,
  outputLevel: SILENT_LEVEL,
  cpuLoadPct: null,
  latencyMs: null,

  play: async (transformId, resourceKey, graph, binaries, params) => {
    teardownSession();
    set({
      status: "loading",
      error: null,
      previewTransformId: transformId,
      previewResourceKey: resourceKey,
      paramValues: params,
      previewGraph: graph,
      nodeParamValues: new Map(graph.executionOrder.map((n) => [n.nodeId, [...n.params]])),
      bypassed: false,
      inputLevel: SILENT_LEVEL,
      outputLevel: SILENT_LEVEL,
      cpuLoadPct: null,
      latencyMs: null,
    });

    try {
      await preview.load(graph, binaries);
    } catch (err) {
      set({ status: "error", error: err instanceof Error ? err.message : String(err) });
      return;
    }

    const audioCtx = preview.audioContext;
    const inputNode = preview.inputNode;
    if (!audioCtx || !inputNode) {
      set({ status: "error", error: "Preview audio context unavailable." });
      return;
    }

    // Test-tone source -> input meter tap -> the preview graph. The input
    // analyser sits inline (it passes audio through unchanged), so this is
    // also the real signal path feeding the transform.
    oscillator = audioCtx.createOscillator();
    oscillator.frequency.value = TEST_TONE_FREQUENCY_HZ;
    inputAnalyser = audioCtx.createAnalyser();
    inputAnalyser.fftSize = 256;
    inputBuffer = new Float32Array(inputAnalyser.fftSize);

    // Output meter tap — a dead-end fork off the preview's processed output,
    // in parallel with (not instead of) preview.load()'s own connection to
    // audioCtx.destination. Not connecting it onward is intentional: it must
    // NOT reach destination a second time, or output would double in volume.
    outputAnalyser = audioCtx.createAnalyser();
    outputAnalyser.fftSize = 256;
    outputBuffer = new Float32Array(outputAnalyser.fftSize);

    oscillator.connect(inputAnalyser);
    inputAnalyser.connect(inputNode);
    inputNode.connect(outputAnalyser);
    oscillator.start();

    unsubscribeCpuLoad = preview.onCpuLoad((value) => set({ cpuLoadPct: value * 100 }));

    // Real, standard Web Audio numbers — not a measurement of the transform's
    // own algorithmic/lookahead latency, which the ABI doesn't expose.
    const latencyMs =
      ((audioCtx.baseLatency ?? 0) + (audioCtx.outputLatency ?? 0)) * 1000 +
      (RENDER_QUANTUM_SIZE / audioCtx.sampleRate) * 1000;

    set({ status: "playing", latencyMs });

    lastMeterUpdate = 0;
    const tick = (time: number) => {
      if (time - lastMeterUpdate >= METER_UPDATE_INTERVAL_MS) {
        lastMeterUpdate = time;
        if (inputAnalyser && inputBuffer && outputAnalyser && outputBuffer) {
          set({
            inputLevel: readLevel(inputAnalyser, inputBuffer),
            outputLevel: readLevel(outputAnalyser, outputBuffer),
          });
        }
      }
      rafHandle = requestAnimationFrame(tick);
    };
    rafHandle = requestAnimationFrame(tick);
  },

  stop: () => {
    teardownSession();
    set({
      status: "idle",
      previewTransformId: null,
      previewResourceKey: null,
      bypassed: false,
      previewGraph: null,
      nodeParamValues: new Map(),
      inputLevel: SILENT_LEVEL,
      outputLevel: SILENT_LEVEL,
      cpuLoadPct: null,
      latencyMs: null,
    });
  },

  setBypass: (bypass) => {
    preview.setBypass(bypass);
    set({ bypassed: bypass });
  },

  updateParam: (index, value) => {
    const next = get().paramValues.slice();
    next[index] = value;
    // The primitive "Try it" preview graph is always the single-node
    // buildPrimitivePreviewGraph, always at executionOrder index 0.
    preview.updateParams(0, next);
    set({ paramValues: next });
  },

  updateNodeParam: (nodeId, paramIndex, value) => {
    const state = get();
    const graph = state.previewGraph;
    if (!graph) return;
    // Looked up fresh against the live graph every call — see the
    // updateNodeParam doc comment on why this can't be cached.
    const nodeIndex = graph.executionOrder.findIndex((n) => n.nodeId === nodeId);
    if (nodeIndex === -1) return;
    const current = state.nodeParamValues.get(nodeId);
    if (!current) return;
    const next = current.slice();
    next[paramIndex] = value;
    preview.updateParams(nodeIndex, next);
    const nodeParamValues = new Map(state.nodeParamValues);
    nodeParamValues.set(nodeId, next);
    set({ nodeParamValues });
  },
}));
