# Skill: Debug / Playback Desync

Use this workflow when waveform UI, graph state, and runtime playback timing drift out of sync.

## Trigger

- seek/play/pause behavior feels wrong
- waveform display and audible result disagree
- graph execution timing appears correct in one layer but not another

## Checklist

1. Identify the boundary where desync appears:
   - editor state
   - Wavesurfer display
   - execution-plan derivation
   - worklet runtime timing
   - persistence echo or reload
2. Reproduce with the smallest possible track, region, and graph.
3. Check lifecycle cleanup and duplicate subscription paths.
4. Check seek/play/pause/stop semantics across the affected boundary.
5. Confirm whether the bug is visual, stateful, or audio-output level.

## High-risk areas

- duplicate event listeners
- stale normalized store state
- mismatched graph plan vs worklet execution order
- non-throttled synchronization loops
- inconsistent time bases between waveform and runtime

## Minimum verification

- reproduce before fix
- verify the same scenario after fix
- verify no regression in play, pause, stop, and seek
