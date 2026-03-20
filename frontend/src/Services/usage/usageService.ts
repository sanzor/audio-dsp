import { http } from "../http";

export type Usage = {
  id: number;
  user_id: number;
  project_count: number;
  total_track_count: number;
  total_storage_bytes: number;
  updated_at: string;
};

export async function getMyUsage(): Promise<Usage> {
  return http.get<Usage>("/v1/usage/mine");
}
