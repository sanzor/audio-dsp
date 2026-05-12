/**
 * AudioEngine
 *
 * Manages the AudioContext, AudioWorkletNode, and AudioBufferSourceNode.
 * Responsible only for audio routing — graph control goes through
 * WorkletMessageSender, playback commands go through StreamPlayer.
 *
 * After init(), call getWorkletPort() to get the MessagePort for
 * WorkletMessageSender.
 */

const WORKLET_URL = new URL('../audio/worklet/graph-worklet.js', import.meta.url).href;

export class AudioEngine {
  private ctx:         AudioContext | null          = null;
  private workletNode: AudioWorkletNode | null      = null;
  private sourceNode:  AudioBufferSourceNode | null = null;
  private offset:      number                       = 0;
  private startedAt:   number                       = 0;

  async init(): Promise<void> {
    if (this.ctx) return;
    this.ctx = new AudioContext();
    await this.ctx.audioWorklet.addModule(WORKLET_URL);
    this.workletNode = new AudioWorkletNode(this.ctx, 'graph-worklet');
    this.workletNode.connect(this.ctx.destination);
  }

  getWorkletPort(): MessagePort {
    if (!this.workletNode) throw new Error('AudioEngine not initialised — call init() first');
    return this.workletNode.port;
  }

  getContext(): AudioContext {
    if (!this.ctx) throw new Error('AudioEngine not initialised — call init() first');
    return this.ctx;
  }

  async play(audioBuffer: AudioBuffer): Promise<void> {
    if (!this.ctx || !this.workletNode) await this.init();
    const ctx = this.ctx!;

    if (ctx.state === 'suspended') await ctx.resume();
    this._stopSource();

    const source  = ctx.createBufferSource();
    source.buffer = audioBuffer;
    source.connect(this.workletNode!);
    source.start(0, this.offset);
    source.onended = () => { this.offset = 0; };

    this.sourceNode = source;
    this.startedAt  = ctx.currentTime - this.offset;
  }

  pause(): void {
    if (!this.ctx || !this.sourceNode) return;
    this.offset = this.ctx.currentTime - this.startedAt;
    this._stopSource();
  }

  stop(): void {
    this._stopSource();
    this.offset = 0;
  }

  destroy(): void {
    this._stopSource();
    this.workletNode?.disconnect();
    this.ctx?.close();
    this.ctx         = null;
    this.workletNode = null;
  }

  private _stopSource(): void {
    try { this.sourceNode?.stop(); } catch { /* already stopped */ }
    this.sourceNode?.disconnect();
    this.sourceNode = null;
  }
}
