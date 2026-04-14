/**
 * AudioCommandDispatcher — routes typed AudioCommands to the right handler.
 *
 * PLAY / STOP / PAUSE  → playback handlers (main thread, AudioBufferSourceNode)
 * SET_GRAPH            → WorkletMessageSender.sendGraph()
 * SET_BYPASS           → WorkletMessageSender.sendBypass()
 * UPDATE_PARAMS        → WorkletMessageSender.sendUpdateParams()
 *
 * Callers (hooks, UI) only ever call dispatch(command) — they never talk to
 * the worklet port or the playback layer directly.
 *
 * StreamPlayer will implement AudioPlaybackHandlers once it is built.
 */

import type { AudioCommand } from './dto/AudioCommand';
import type { CompiledGraph } from './dto/CompiledGraph';
import { WorkletMessageSender } from './WorkletMessageSender';

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
