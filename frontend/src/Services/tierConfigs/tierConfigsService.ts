import { http } from "../http";

export type TierConfig = {
  id: number;
  tier: string;
  max_projects: number;
  max_tracks_per_project: number;
  max_storage_bytes: number;
  updated_at: string;
};

export async function listTierConfigs(): Promise<TierConfig[]> {
  return http.get<TierConfig[]>("/v1/tier-configs");
}
