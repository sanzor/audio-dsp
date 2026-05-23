import { WorkletMessageSender } from './WorkletMessageSender';
import type { CompiledGraphResult } from '@/audio/pipeline/compileRuntimeGraph';

export class WorkletController {
  private sender: WorkletMessageSender | null = null;

  connectWorklet(port: MessagePort): WorkletMessageSender {
    const sender = new WorkletMessageSender(port);
    this.sender = sender;
    return sender;
  }

  disconnectWorklet(): void {
    this.sender = null;
  }

  loadCompiledGraphToWorklet(descriptor: CompiledGraphResult): void {
    this.sender?.sendGraph(descriptor.compiledGraph, descriptor.resolved_binaries);
  }

  setEffects(enabled: boolean): void {
    this.sender?.sendBypass(!enabled);
  }
}
