
import type { CanonicalAudio } from "./CanonicalAudio";
import type { ABuffer } from "@/Domain/ABuffer";

export async function toCanonicalAudio(
  input: File | Float32Array[], // file (upload) OR raw samples (recorded)
  sampleRate?: number // required if input is raw
): Promise<CanonicalAudio> {
  if (input instanceof File) {
    // File case
    const arrayBuffer = await input.arrayBuffer();
    const audioCtx = new AudioContext();
    const buffer = await audioCtx.decodeAudioData(arrayBuffer);
    
    return {
      sampleRate: buffer.sampleRate,
      channels: buffer.numberOfChannels,
      duration: buffer.duration,
      samples: Array.from({ length: buffer.numberOfChannels }, (_, i) =>
            new Float32Array(buffer.getChannelData(i)) // <- makes a copy
      ),
    };
  } else {
    // Raw Float32Array[] case (from recorder)
    if (!sampleRate) throw new Error("Sample rate is required for raw audio input");

    const duration = input[0].length / sampleRate;

    return {
      sampleRate,
      channels: input.length,
      duration,
      samples: input,
    };
  }
}

export function toAudioBuffer(canonical: CanonicalAudio): ABuffer {
  const samples: number[] = canonical.samples.flatMap((buffer) => Array.from(buffer));

  return {
    samples,
    sample_rate: canonical.sampleRate,
    channels: canonical.channels,
  };
}

export function interleave(buffers: Float32Array[]): Float32Array {
  const length = buffers[0].length
  const channels = buffers.length
  const result = new Float32Array(length * channels)

  for (let i = 0; i < length; i++) {
    for (let c = 0; c < channels; c++) {
      result[i * channels + c] = buffers[c][i]
    }
  }

  return result
}

// encodeWav.ts
export function encodeWav(samples: Float32Array, sampleRate: number, channels: number): Blob {
  const buffer = new ArrayBuffer(44 + samples.length * 2)
  const view = new DataView(buffer)

  function writeStr(offset: number, str: string) {
    for (let i = 0; i < str.length; i++) {
      view.setUint8(offset + i, str.charCodeAt(i))
    }
  }

  writeStr(0, "RIFF")
  view.setUint32(4, 36 + samples.length * 2, true)
  writeStr(8, "WAVE")
  writeStr(12, "fmt ")
  view.setUint32(16, 16, true)
  view.setUint16(20, 1, true)
  view.setUint16(22, channels, true)
  view.setUint32(24, sampleRate, true)
  view.setUint32(28, sampleRate * channels * 2, true)
  view.setUint16(32, channels * 2, true)
  view.setUint16(34, 16, true)
  writeStr(36, "data")
  view.setUint32(40, samples.length * 2, true)

  for (let i = 0; i < samples.length; i++) {
    const s = Math.max(-1, Math.min(1, samples[i]))
    view.setInt16(44 + i * 2, s < 0 ? s * 0x8000 : s * 0x7FFF, true)
  }

  return new Blob([view], { type: "audio/wav" })
}