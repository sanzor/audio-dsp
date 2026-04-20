export class TransformStore {
  private readonly cache = new Map<string, ArrayBuffer>()

  async get(transformId: string): Promise<ArrayBuffer> {
    const cached = this.cache.get(transformId)
    if (cached) return cached

    const response = await fetch(`/api/transforms/${transformId}/binary`)
    if (!response.ok) throw new Error(`Failed to fetch transform ${transformId}: ${response.status}`)

    const buffer = await response.arrayBuffer()
    this.cache.set(transformId, buffer)
    return buffer
  }

  async preload(transformIds: string[]): Promise<void> {
    await Promise.all(transformIds.map(id => this.get(id)))
  }

  invalidate(transformId: string): void {
    this.cache.delete(transformId)
  }
}
