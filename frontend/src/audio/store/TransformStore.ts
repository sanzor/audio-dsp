import { API_BASE_URL } from "@/Services/http"
import { useAuthStore } from "@/Stores/authStore"
import { useProjectStore } from "@/Stores/projectStore"

export class TransformStore {
  private readonly cache = new Map<string, ArrayBuffer>()

  async get(transformId: string): Promise<ArrayBuffer> {
    const cached = this.cache.get(transformId)
    if (cached) return cached

    const token = useAuthStore.getState().token ?? undefined
    const activeProjectId = useProjectStore.getState().activeProject?.project_id
    const response = await fetch(`${API_BASE_URL}/transforms/${transformId}/binary`, {
      headers: {
        ...(token ? { Authorization: `Bearer ${token}` } : {}),
        ...(activeProjectId != null ? { "X-Project-Id": String(activeProjectId) } : {}),
      },
    })
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
