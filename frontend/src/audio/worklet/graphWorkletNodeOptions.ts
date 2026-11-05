// The single source of truth for how a `graph-worklet` AudioWorkletNode is
// constructed. Both the editor (useWorkletSetup.ts) and the creator "Try it"
// preview (creatorTransformPlayback.ts) must use this, so preview can't
// diverge from post-publish playback (see agents/invariants.md).
//
// The channel layout is explicit, per the audio-correctness invariant:
// exactly 2 channels in and out with `speakers` interpretation. A mono
// source is upmixed to L=R by the browser and anything wider is downmixed to
// stereo before it reaches process(). Must stay in sync with CHANNEL_COUNT
// in graph-worklet.js, which runs one processing lane per channel.
export const GRAPH_WORKLET_CHANNEL_COUNT = 2;

export const GRAPH_WORKLET_NODE_OPTIONS: AudioWorkletNodeOptions = {
  numberOfInputs: 1,
  numberOfOutputs: 1,
  outputChannelCount: [GRAPH_WORKLET_CHANNEL_COUNT],
  channelCount: GRAPH_WORKLET_CHANNEL_COUNT,
  channelCountMode: "explicit",
  channelInterpretation: "speakers",
};
