import type { RuntimeStatus } from "@/Stores/AudioEffectsStore"
import { useAudioEffectsStore } from "@/Stores/AudioEffectsStore"

export interface RuntimeStateProvider {
  setRuntimeState(status: RuntimeStatus, msg?: string | null): void
}

export class AudioEffectsRuntimeStateProvider implements RuntimeStateProvider {
  setRuntimeState(status: RuntimeStatus, msg?: string | null): void {
    useAudioEffectsStore.getState().setRuntimeState(status, msg)
  }
}
