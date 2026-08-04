// "Try it" playback runtime for the creator surface — runs a just-compiled
// (not yet saved/published) transform binary client-side, before the
// creator decides to save it. See agents/decisions/0003-transform-preview-flow.md
// and the "Try it" section of agents/transforms.md.
//
// This is a *narrow, creator-scoped* wrapper directly on top of the editor's
// worklet module and message-port protocol — not a hand-rolled runtime and
// not a reuse of the editor's stateful WorkletController. Both constraints
// are load-bearing (see agents/ownership.md's Shared zones and
// agents/architecture.md's Creator/Editor boundary):
//
//   - Safe to reuse directly: `graph-worklet.js` (the AudioWorkletProcessor
//     itself — no module-level shared state, each AudioWorkletNode instance
//     is independent) and `WorkletMessageSender` (a plain port-message
//     wrapper, no store coupling). Reusing these means playback executes
//     under the *identical* ABI/calling-convention the editor uses
//     post-publish, so this "Try it" playback can't silently diverge from
//     production.
//   - Deliberately NOT reused: `WorkletController` (a hard singleton at
//     module scope in Stores/WorkletStore.ts that writes into editor-only
//     global Zustand state — useWorkletStore/useAudioEffectsStore — on every
//     connect/graph-ready/error event) and `useWorkletSetup`/`useWorklet`
//     (wired to a Wavesurfer media element the creator surface doesn't
//     have). Reusing either would corrupt editor playback state once both
//     surfaces are visited in one session.
import { WorkletMessageSender } from "@/audio/transport/WorkletMessageSender";
import type { CompiledGraph } from "@/audio/pipeline/compile-graph/compiledGraph";
import type { CompiledNode } from "@/audio/pipeline/GraphCompiler";

const GRAPH_WORKLET_URL = new URL("../../audio/worklet/graph-worklet.js", import.meta.url).href;

// The worklet keys binaries by transformId (see WorkletMessageSender.sendGraph
// and graph-worklet.js's onSetGraph). A "Try it" playback binary has no real
// transform_id yet (it hasn't been saved/published), so any stable
// placeholder works here — it never leaves this module.
export const PRIMITIVE_PLAYBACK_NODE_ID = -1;

// Builds the degenerate one-node graph used to play back a single
// just-compiled primitive transform in isolation. Exported so the primitive
// playback path (CreatorPlaybackStore's `play` for code-editor.tsx) can build
// its CompiledGraph the same way `load()` used to build it internally —
// `load()` itself is now graph-shape-agnostic (see below) to also support
// the composite canvas's multi-node playback.
export function buildPrimitivePlaybackGraph(params: number[]): CompiledGraph {
  const node: CompiledNode = {
    nodeId: PRIMITIVE_PLAYBACK_NODE_ID,
    transformId: PRIMITIVE_PLAYBACK_NODE_ID,
    params,
    inputs: [[{ kind: "raw" }]],
    outputBufferIndex: -1, // no forward/back edges — no buffer needed
    writesToOutput: true, // always-terminal playback node — writes straight to worklet output
  };
  return {
    executionOrder: [node],
    bufferCount: 0,
    feedbackBufferIndices: [],
  };
}

// Creator-scoped connect/disconnect wrapper. One instance per "Try it"
// session (e.g. per code-editor mount) — construct, call load(), tear down
// with dispose() on unmount or when switching to a different transform.
export class CreatorTransformPlayback {
  private audioCtx: AudioContext | null = null;
  private node: AudioWorkletNode | null = null;
  private sender: WorkletMessageSender | null = null;

  private async ensureConnected(): Promise<{ audioCtx: AudioContext; node: AudioWorkletNode; sender: WorkletMessageSender }> {
    if (this.audioCtx && this.node && this.sender) {
      return { audioCtx: this.audioCtx, node: this.node, sender: this.sender };
    }

    const audioCtx = new AudioContext();
    await audioCtx.audioWorklet.addModule(GRAPH_WORKLET_URL);
    const node = new AudioWorkletNode(audioCtx, "graph-worklet");
    const sender = new WorkletMessageSender(node.port);

    this.audioCtx = audioCtx;
    this.node = node;
    this.sender = sender;

    return { audioCtx, node, sender };
  }

  // Loads an arbitrary CompiledGraph (a single primitive transform via
  // buildPrimitivePlaybackGraph, or a multi-node composite-in-progress graph
  // compiled by GraphCompiler.process) and connects the worklet node to the
  // audio destination so it's immediately audible once some source is
  // routed into it by the caller (see `inputNode` below). `binaries` is
  // keyed by transformId, matching graph-worklet.js's onSetGraph lookup —
  // graph-worklet.js itself needs no changes to run either shape.
  async load(graph: CompiledGraph, binaries: Record<number, Uint8Array>): Promise<void> {
    const { audioCtx, node, sender } = await this.ensureConnected();
    if (audioCtx.state === "suspended") await audioCtx.resume();

    const ready = new Promise<void>((resolve, reject) => {
      const unsubReady = sender.onGraphReady(() => {
        unsubReady();
        unsubError();
        resolve();
      });
      const unsubError = sender.onModuleError((_transformId, error) => {
        unsubReady();
        unsubError();
        reject(new Error(error));
      });
    });

    sender.sendGraph(graph, binaries);
    await ready;

    node.connect(audioCtx.destination);
    sender.sendBypass(false);
  }

  // Exposes the worklet node so the caller can route a test signal (an
  // oscillator, a decoded sample, a mic input — the creator surface's
  // choice, not this wrapper's) into the playback graph's input.
  get inputNode(): AudioWorkletNode | null {
    return this.node;
  }

  // Exposes the underlying AudioContext so the caller can build supporting
  // nodes (a test-tone oscillator, level-metering analysers) that must live
  // in the same context as inputNode.
  get audioContext(): AudioContext | null {
    return this.audioCtx;
  }

  setBypass(bypass: boolean): void {
    this.sender?.sendBypass(bypass);
  }

  // `nodeIndex` is the target node's position in the *currently loaded*
  // CompiledGraph.executionOrder — not its node_id. The primitive playback
  // graph (buildPrimitivePlaybackGraph) always has exactly one node at index
  // 0, so CreatorPlaybackStore.updateParam (the primitive "Try it" path)
  // passes 0 explicitly. The composite canvas's per-node param editing
  // (CreatorPlaybackStore.updateNodeParam) looks its node's index up fresh
  // against the live playback graph on every call instead — see
  // agents/decisions/0005-composite-node-inspector.md — since which nodes
  // even have an index at all changes as disabled nodes are compiled out.
  updateParams(nodeIndex: number, params: number[]): void {
    this.sender?.sendUpdateParams(nodeIndex, params);
  }

  onCpuLoad(cb: (value: number) => void): () => void {
    return this.sender?.onCpuLoad(cb) ?? (() => {});
  }

  // Tears down cleanly — safe to call multiple times, and required on
  // unmount/transform-switch per the audio-lifecycle invariant in
  // agents/invariants.md ("Any lifecycle around Wavesurfer or playback
  // resources must clean up deterministically").
  dispose(): void {
    this.sender?.dispose();
    this.sender = null;
    this.node?.disconnect();
    this.node = null;
    void this.audioCtx?.close();
    this.audioCtx = null;
  }
}
