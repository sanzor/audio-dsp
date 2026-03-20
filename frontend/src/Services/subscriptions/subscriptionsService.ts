import { http } from "../http";

export type Subscription = {
  id: number;
  user_id: number;
  tier: string;
  is_active: boolean;
  started_at: string;
  expires_at: string | null;
};

export async function listMySubscriptions(): Promise<Subscription[]> {
  return http.get<Subscription[]>("/v1/subscriptions/mine");
}

export async function getMyActiveSubscription(): Promise<Subscription | null> {
  try {
    return await http.get<Subscription>("/v1/subscriptions/mine/active");
  } catch {
    return null;
  }
}
