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
    preview.updateParams(next);
    set({ paramValues: next });
  },
}));
