// "Try it" preview runtime for the creator surface — runs a just-compiled
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
//     wrapper, no store coupling). Reusing these means preview executes
//     under the *identical* ABI/calling-convention the editor uses
//     post-publish, so preview can't silently diverge from production.
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
// and graph-worklet.js's onSetGraph). A preview binary has no real
// transform_id yet (it hasn't been saved/published), so any stable
// placeholder works here — it never leaves this module.
const PREVIEW_NODE_ID = -1;

function buildSingleNodeGraph(params: number[]): CompiledGraph {
  const node: CompiledNode = {
    nodeId: PREVIEW_NODE_ID,
    transformId: PREVIEW_NODE_ID,
    params,
    inputs: [{ kind: "raw" }],
    outputBufferIndex: -1, // sink — writes straight to worklet output
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
export class CreatorTransformPreview {
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

  // Loads `wasmBytes` into a degenerate one-node graph and connects the
  // worklet node to the audio destination so it's immediately audible once
  // some source is routed into it by the caller (see `inputNode` below).
  async load(wasmBytes: Uint8Array, params: number[] = []): Promise<void> {
    const { audioCtx, node, sender } = await this.ensureConnected();
    if (audioCtx.state === "suspended") await audioCtx.resume();

    const graph = buildSingleNodeGraph(params);

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

    sender.sendGraph(graph, { [PREVIEW_NODE_ID]: wasmBytes });
    await ready;

    node.connect(audioCtx.destination);
    sender.sendBypass(false);
  }

  // Exposes the worklet node so the caller can route a test signal (an
  // oscillator, a decoded sample, a mic input — the creator surface's
  // choice, not this wrapper's) into the preview graph's input.
  get inputNode(): AudioWorkletNode | null {
    return this.node;
  }

  setBypass(bypass: boolean): void {
    this.sender?.sendBypass(bypass);
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
