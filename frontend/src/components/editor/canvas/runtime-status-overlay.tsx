import { useAudioEffectsStore } from "@/Stores/AudioEffectsStore";
import { useWorkletStore } from "@/Stores/WorkletStore";
import { useActiveGraphId } from "@/hooks/graphs/useActiveGraphId";

export function RuntimeStatusOverlay() {
  const activeGraphId = useActiveGraphId();
  const runtimeStatus = useAudioEffectsStore((s) => s.runtimeStatus);
  const runtimeMessage = useAudioEffectsStore((s) => s.runtimeMessage);
  const workletConnected = useWorkletStore((s) => s.workletConnected);
  const effectsEnabled = useWorkletStore((s) => s.effectsEnabled);
  const graphPlaybackState = useWorkletStore((s) => s.graphPlaybackState);
  const isGraphPlayable = useWorkletStore((s) => s.isGraphPlayable);

  return (
    <div
      className="pointer-events-none absolute right-3 top-3 z-20 max-w-sm rounded-md border px-3 py-2 text-xs"
      style={{
        background: "rgba(15, 23, 42, 0.9)",
        borderColor:
          activeGraphId == null
            ? "rgba(255,255,255,0.12)"
            : isGraphPlayable()
              ? "rgba(74, 222, 128, 0.45)"
              : "rgba(248, 113, 113, 0.45)",
        color: "var(--text-main)",
      }}
    >
      <div className="font-medium">Runtime: {runtimeStatus}</div>
      <div>Worklet: {workletConnected ? "connected" : "disconnected"}</div>
      <div>Effects: {effectsEnabled ? "enabled" : "bypassed"}</div>
      <div>Compiled: {graphPlaybackState.compiled ? "yes" : "no"}</div>
      <div>Playable: {isGraphPlayable() ? "yes" : "no"}</div>
      {runtimeMessage && <div>{runtimeMessage}</div>}
    </div>
  );
}
