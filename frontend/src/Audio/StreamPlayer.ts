/**
 * StreamPlayer
 *
 * The central audio controller.  Owns the AudioEngine and wires up the
 * AudioCommandDispatcher once the engine is initialised.
 *
 * Lazy initialisation: the AudioContext can only be created after a user
 * gesture.  StreamPlayer queues SET_GRAPH / SET_BYPASS commands received
 * before init, then flushes them once PLAY is first called.
 *
 * Usage:
 *   player.dispatch({ type: 'SET_GRAPH', ... })   ← queued until init
 *   player.dispatch({ type: 'PLAY', audioBuffer }) ← triggers init, flushes queue, plays
 *   player.dispatch({ type: 'PAUSE' })
 *   player.dispatch({ type: 'STOP' })
 */

import { AudioEngine } from './AudioEngine';
import { WorkletMessageSender } from '@/audio/transport/WorkletMessageSender';
import { AudioCommandDispatcher } from '@/audio/transport/AudioCommandDispatcher';
import type { AudioPlaybackHandlers } from '@/audio/transport/AudioCommandDispatcher';
import type { AudioCommand } from '@/audio/transport/AudioCommandDispatcher';

export class StreamPlayer implements AudioPlaybackHandlers {
  private engine:     AudioEngine;
  private sender:     WorkletMessageSender | null     = null;
  private dispatcher: AudioCommandDispatcher | null   = null;
  private queue:      AudioCommand[]                  = [];
  private initialised = false;

  constructor() {
    this.engine = new AudioEngine();
  }

  dispatch(command: AudioCommand): void {
    if (!this.initialised) {
      if (command.type === 'PLAY') {
        void this.initThenFlush(command);
      } else {
        this.queue.push(command); // hold until engine is ready
      }
      return;
    }
    this.dispatcher!.dispatch(command);
  }

  destroy(): void {
    this.engine.destroy();
    this.sender     = null;
    this.dispatcher = null;
    this.queue      = [];
    this.initialised = false;
  }

  // ── AudioPlaybackHandlers ───────────────────────────────────────────────────

  async onPlay(audioBuffer: AudioBuffer): Promise<void> {
    await this.engine.play(audioBuffer);
  }

  onPause(): void {
    this.engine.pause();
  }

  onStop(): void {
    this.engine.stop();
  }

  // ── Private ─────────────────────────────────────────────────────────────────

  private async initThenFlush(playCommand: AudioCommand): Promise<void> {
    await this.engine.init();

    this.sender     = new WorkletMessageSender(this.engine.getWorkletPort());
    this.dispatcher = new AudioCommandDispatcher(this, this.sender);
    this.initialised = true;

    // Flush queued graph/bypass commands first so the worklet is configured
    // before audio starts flowing through it.
    for (const queued of this.queue) {
      this.dispatcher.dispatch(queued);
    }
    this.queue = [];

    this.dispatcher.dispatch(playCommand);
  }
}
