import type { CompiledGraph } from '@/audio/pipeline/GraphCompiler';
import { WorkletMessageSender } from './WorkletMessageSender';

// ─── Command types ────────────────────────────────────────────────────────────

export type AudioCommand =
  | { type: 'PLAY';          audioBuffer: AudioBuffer }
  | { type: 'STOP' }
  | { type: 'PAUSE' }
  | { type: 'SET_GRAPH';     compiledGraph: CompiledGraph; binaries: Record<number, Uint8Array> }
  | { type: 'SET_BYPASS';    bypass: boolean }
  | { type: 'UPDATE_PARAMS'; nodeIndex: number; params: number[] };

// ─── Dispatcher ───────────────────────────────────────────────────────────────
//
// Routes AudioCommands to the right handler:
//   PLAY / STOP / PAUSE  → AudioPlaybackHandlers (main thread)
//   SET_GRAPH / SET_BYPASS / UPDATE_PARAMS → WorkletMessageSender

export interface AudioPlaybackHandlers {
  onPlay(audioBuffer: AudioBuffer): void | Promise<void>;
  onStop(): void;
  onPause(): void;
}

export class AudioCommandDispatcher {
  constructor(
    private playback: AudioPlaybackHandlers,
    private sender:   WorkletMessageSender,
  ) {}

  dispatch(command: AudioCommand): void {
    switch (command.type) {
      case 'PLAY':
        void this.playback.onPlay(command.audioBuffer);
        break;
      case 'STOP':
        this.playback.onStop();
        break;
      case 'PAUSE':
        this.playback.onPause();
        break;
      case 'SET_GRAPH':
        this.sender.sendGraph(command.compiledGraph, command.binaries);
        break;
      case 'SET_BYPASS':
        this.sender.sendBypass(command.bypass);
        break;
      case 'UPDATE_PARAMS':
        this.sender.sendUpdateParams(command.nodeIndex, command.params);
        break;
    }
  }
}
