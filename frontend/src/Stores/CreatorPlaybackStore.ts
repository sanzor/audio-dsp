import { create } from "zustand";
import { CreatorTransformPlayback } from "@/components/creator/creatorTransformPlayback";
import type { CompiledGraph } from "@/audio/pipeline/compile-graph/compiledGraph";
import { apiGetSourceAudio } from "@/Services/SourceService";

// Shared "Try it" live-audition session for the creator surface. Unpersisted
// — mirrors WorkletStore.ts's shape (a module-scope class instance held
// directly as state, no `persist` middleware) rather than CreatorStore.ts's
// persisted store, since none of this is meant to survive a reload.
//
// Raw Web Audio objects (oscillator, analysers, the rAF handle, the CPU-load
// unsubscribe fn) are module-scope `let`s, not store state — only the derived
// numbers components actually render (levels, cpu%, latency) go through
// Zustand, so subscribers re-render on data changes, not on node churn.

export type PlaybackStatus = "idle" | "loading" | "playing" | "error";

// "tone" is the original synthetic 440Hz test tone; "source" feeds
// playback from an uploaded Creator-surface signal source instead (see
// backend/domain/src/sources/source_info.rs). Selecting a source is
// possible whether or not a playback session is currently running -- if one
// is, the input is hot-swapped in place without reloading the worklet graph
// (see attachInputSource below); if not, the choice is picked up by the
// next play().
export type PlaybackInputMode = "tone" | "source";

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

const playback = new CreatorTransformPlayback();

// Whatever's currently feeding inputAnalyser -- an OscillatorNode (tone
// mode) or an AudioBufferSourceNode (source mode). Both implement
// AudioScheduledSourceNode's start()/stop(), so attachInputSource can treat
// them uniformly.
let inputSourceNode: AudioScheduledSourceNode | null = null;
let inputAnalyser: AnalyserNode | null = null;
let outputAnalyser: AnalyserNode | null = null;
let inputBuffer: Float32Array | null = null;
let outputBuffer: Float32Array | null = null;
let rafHandle: number | null = null;
let lastMeterUpdate = 0;
let unsubscribeCpuLoad: (() => void) | null = null;

// Decoded source audio, keyed by source_id -- avoids re-fetching/re-decoding
// on every play() or input hot-swap. AudioBuffer isn't tied to the
// AudioContext that decoded it, so it's safe to reuse across playback
// sessions (each of which gets a fresh AudioContext).
const decodedSourceCache = new Map<number, AudioBuffer>();

async function getDecodedSourceBuffer(audioCtx: AudioContext, sourceId: number): Promise<AudioBuffer> {
  const cached = decodedSourceCache.get(sourceId);
  if (cached) return cached;
  const blob = await apiGetSourceAudio(sourceId);
  const arrayBuffer = await blob.arrayBuffer();
  const decoded = await audioCtx.decodeAudioData(arrayBuffer);
  decodedSourceCache.set(sourceId, decoded);
  return decoded;
}

// Builds and starts whatever should currently feed `inputAnalyser` --
// re-reads playbackInputMode/playbackSourceId from the store fresh on every
// call, so it works both for the initial connect in play() and for a live
// hot-swap triggered by setPlaybackInputMode/setPlaybackInputSource while a
// session is already running. Stops/disconnects the previous source first;
// leaves inputAnalyser itself (and its downstream connection to the worklet
// input node) untouched -- only the upstream source swaps.
async function attachInputSource(audioCtx: AudioContext, analyser: AnalyserNode): Promise<void> {
  try {
    inputSourceNode?.stop();
  } catch {
    // Already stopped or never started -- fine either way.
  }
  inputSourceNode?.disconnect();
  inputSourceNode = null;

  const { playbackInputMode, playbackSourceId } = useCreatorPlaybackStore.getState();

  if (playbackInputMode === "source" && playbackSourceId != null) {
    try {
      const buffer = await getDecodedSourceBuffer(audioCtx, playbackSourceId);
      const { playbackSourceOffsetSeconds } = useCreatorPlaybackStore.getState();
      const offset = Math.min(Math.max(playbackSourceOffsetSeconds, 0), buffer.duration);
      const bufferSource = audioCtx.createBufferSource();
      bufferSource.buffer = buffer;
      bufferSource.loop = true;
      // Judgment call (see playback-waveform.tsx/playback-transport.tsx):
      // anchor the loop at the chosen offset, not the buffer start. Web
      // Audio's default loopStart/loopEnd is [0, duration] regardless of
      // where start(when, offset) begins playing, so without this the first
      // pass plays offset..end but every subsequent pass replays the whole
      // buffer from 0 -- audibly inconsistent with "play from here". Setting
      // loopStart = offset makes every pass (first and looped) cover the
      // same offset..end tail, matching what picking a start point on the
      // waveform implies and staying consistent with what
      // setPlaybackSourceOffset produces on a live hot-swap.
      bufferSource.loopStart = offset;
      bufferSource.connect(analyser);
      bufferSource.start(0, offset);
      inputSourceNode = bufferSource;
      useCreatorPlaybackStore.setState({ playbackInputError: null });
      return;
    } catch (err) {
      // Falls back to the test tone below -- a source that fails to fetch
      // or decode must not leave the playback session silently disconnected.
      useCreatorPlaybackStore.setState({
        playbackInputError: err instanceof Error ? err.message : String(err),
      });
    }
  }

  const osc = audioCtx.createOscillator();
  osc.frequency.value = TEST_TONE_FREQUENCY_HZ;
  osc.connect(analyser);
  osc.start();
  inputSourceNode = osc;
}

interface CreatorPlaybackState {
  playbackTransformId: number | null;
  // Opaque staleness key each caller builds itself — primitives use
  // `${resourceId}:${sourceCode}`, composites a stringified hash of the
  // current graph state. Keeps this store agnostic to which kind of
  // transform it's playing back.
  playbackResourceKey: string | null;
  status: PlaybackStatus;
  error: string | null;
  bypassed: boolean;
  paramValues: number[];
  // The exact CompiledGraph currently loaded into the worklet — kept around
  // (not just consumed and discarded at play() time) so composite-canvas.tsx
  // can look up a node's live executionOrder index by node_id on demand, at
  // the moment of each param edit. Null whenever nothing is playing back.
  playbackGraph: CompiledGraph | null;
  // Per-node live param values for the currently loaded playback graph, keyed
  // by node_id (== CompiledNode.nodeId — for the single-node primitive
  // playback graph that's PRIMITIVE_PLAYBACK_NODE_ID). Separate from the flat
  // `paramValues` above, which is the primitive "Try it" path's own
  // single-transform param state and is left untouched by this addition.
  nodeParamValues: Map<number, number[]>;
  inputLevel: LevelReading;
  outputLevel: LevelReading;
  cpuLoadPct: number | null;
  latencyMs: number | null;
  // Persists across play()/stop() and across switching which transform is
  // selected -- this is a session-wide choice ("what am I auditioning
  // with"), not per-transform state, matching this store's shared-session
  // framing (see the file-level doc comment above).
  playbackInputMode: PlaybackInputMode;
  playbackSourceId: number | null;
  // Where in the selected source's buffer playback starts, in seconds --
  // chosen by clicking/dragging on playback-waveform.tsx. Only meaningful in
  // "source" mode; ignored (and left as-is) in "tone" mode. Session-wide
  // like playbackInputMode/playbackSourceId above, not per-transform.
  playbackSourceOffsetSeconds: number;
  // Set when "source" mode is selected but fetching/decoding that source's
  // audio failed -- the session falls back to the test tone rather than
  // going silent, and this surfaces why for the input selector UI.
  playbackInputError: string | null;
  setPlaybackInputMode: (mode: PlaybackInputMode) => void;
  setPlaybackInputSource: (sourceId: number | null) => void;
  setPlaybackSourceOffset: (seconds: number) => void;
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
  // `nodeId` to its current executionOrder index against `playbackGraph`
  // fresh on every call — deliberately not cached/memoized, since Phase 3's
  // disabled-node filtering changes which nodes are even present in the
  // compiled graph between compiles. No-ops if the node isn't in the
  // currently loaded playback graph (not playing, or filtered out as
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
    inputSourceNode?.stop();
  } catch {
    // Already stopped or never started — fine either way.
  }
  inputSourceNode?.disconnect();
  inputSourceNode = null;
  inputAnalyser?.disconnect();
  inputAnalyser = null;
  outputAnalyser?.disconnect();
  outputAnalyser = null;
  inputBuffer = null;
  outputBuffer = null;

  playback.dispose();
}

export const useCreatorPlaybackStore = create<CreatorPlaybackState>()((set, get) => ({
  playbackTransformId: null,
  playbackResourceKey: null,
  status: "idle",
  error: null,
  bypassed: false,
  paramValues: [],
  playbackGraph: null,
  nodeParamValues: new Map(),
  inputLevel: SILENT_LEVEL,
  outputLevel: SILENT_LEVEL,
  cpuLoadPct: null,
  latencyMs: null,
  playbackInputMode: "tone",
  playbackSourceId: null,
  playbackSourceOffsetSeconds: 0,
  playbackInputError: null,

  setPlaybackInputMode: (mode) => {
    set({ playbackInputMode: mode, playbackInputError: null });
    const audioCtx = playback.audioContext;
    if (get().status === "playing" && audioCtx && inputAnalyser) {
      void attachInputSource(audioCtx, inputAnalyser);
    }
  },

  setPlaybackInputSource: (sourceId) => {
    set({ playbackSourceId: sourceId, playbackSourceOffsetSeconds: 0, playbackInputError: null });
    const audioCtx = playback.audioContext;
    if (get().status === "playing" && get().playbackInputMode === "source" && audioCtx && inputAnalyser) {
      void attachInputSource(audioCtx, inputAnalyser);
    }
  },

  setPlaybackSourceOffset: (seconds) => {
    set({ playbackSourceOffsetSeconds: Math.max(0, seconds) });
    const audioCtx = playback.audioContext;
    if (get().status === "playing" && get().playbackInputMode === "source" && audioCtx && inputAnalyser) {
      void attachInputSource(audioCtx, inputAnalyser);
    }
  },

  play: async (transformId, resourceKey, graph, binaries, params) => {
    teardownSession();
    set({
      status: "loading",
      error: null,
      playbackTransformId: transformId,
      playbackResourceKey: resourceKey,
      paramValues: params,
      playbackGraph: graph,
      nodeParamValues: new Map(graph.executionOrder.map((n) => [n.nodeId, [...n.params]])),
      bypassed: false,
      inputLevel: SILENT_LEVEL,
      outputLevel: SILENT_LEVEL,
      cpuLoadPct: null,
      latencyMs: null,
    });

    try {
      await playback.load(graph, binaries);
    } catch (err) {
      set({ status: "error", error: err instanceof Error ? err.message : String(err) });
      return;
    }

    const audioCtx = playback.audioContext;
    const inputNode = playback.inputNode;
    if (!audioCtx || !inputNode) {
      set({ status: "error", error: "Playback audio context unavailable." });
      return;
    }

    // Input source (test tone, or a selected uploaded source — see
    // attachInputSource) -> input meter tap -> the playback graph. The input
    // analyser sits inline (it passes audio through unchanged), so this is
    // also the real signal path feeding the transform.
    inputAnalyser = audioCtx.createAnalyser();
    inputAnalyser.fftSize = 256;
    inputBuffer = new Float32Array(inputAnalyser.fftSize);

    // Output meter tap — a dead-end fork off the playback's processed output,
    // in parallel with (not instead of) playback.load()'s own connection to
    // audioCtx.destination. Not connecting it onward is intentional: it must
    // NOT reach destination a second time, or output would double in volume.
    outputAnalyser = audioCtx.createAnalyser();
    outputAnalyser.fftSize = 256;
    outputBuffer = new Float32Array(outputAnalyser.fftSize);

    inputAnalyser.connect(inputNode);
    inputNode.connect(outputAnalyser);
    await attachInputSource(audioCtx, inputAnalyser);

    unsubscribeCpuLoad = playback.onCpuLoad((value) => set({ cpuLoadPct: value * 100 }));

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
      playbackTransformId: null,
      playbackResourceKey: null,
      bypassed: false,
      playbackGraph: null,
      nodeParamValues: new Map(),
      inputLevel: SILENT_LEVEL,
      outputLevel: SILENT_LEVEL,
      cpuLoadPct: null,
      latencyMs: null,
    });
  },

  setBypass: (bypass) => {
    playback.setBypass(bypass);
    set({ bypassed: bypass });
  },

  updateParam: (index, value) => {
    const next = get().paramValues.slice();
    next[index] = value;
    // The primitive "Try it" playback graph is always the single-node
    // buildPrimitivePlaybackGraph, always at executionOrder index 0.
    playback.updateParams(0, next);
    set({ paramValues: next });
  },

  updateNodeParam: (nodeId, paramIndex, value) => {
    const state = get();
    const graph = state.playbackGraph;
    if (!graph) return;
    // Looked up fresh against the live graph every call — see the
    // updateNodeParam doc comment on why this can't be cached.
    const nodeIndex = graph.executionOrder.findIndex((n) => n.nodeId === nodeId);
    if (nodeIndex === -1) return;
    const current = state.nodeParamValues.get(nodeId);
    if (!current) return;
    const next = current.slice();
    next[paramIndex] = value;
    playback.updateParams(nodeIndex, next);
    const nodeParamValues = new Map(state.nodeParamValues);
    nodeParamValues.set(nodeId, next);
    set({ nodeParamValues });
  },
}));
