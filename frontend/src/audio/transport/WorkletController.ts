import { WorkletMessageSender } from './WorkletMessageSender';
import type { TransformDescriptor } from '@/audio/pipeline/GraphCompiler';

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

  loadCompiledGraph(descriptor: TransformDescriptor): void {
    this.sender?.sendGraph(descriptor.compiledGraph, descriptor.binaries);
  }

  setEffects(enabled: boolean): void {
    this.sender?.sendBypass(!enabled);
  }
}
