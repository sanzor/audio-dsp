import type { ResolveBinariesParams } from "./resolveBinariesParams";
import type { ResolveBinariesResult } from "./resolveBinariesResult";

export function resolveBinaries(params: ResolveBinariesParams): ResolveBinariesResult | null {
  const binaries: Record<number, Uint8Array> = {}

  for (const transformId of params.nodeTransformMap.values()) {
    if (transformId in binaries) continue
    const binary = params.binaryTransformsMap.get(transformId)
    if (!binary) return null
    binaries[transformId] = binary
  }

  return { binaries }
}
