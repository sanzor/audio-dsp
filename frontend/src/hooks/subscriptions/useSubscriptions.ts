import { useQuery } from "@tanstack/react-query";
import {
  listMySubscriptions,
  getMyActiveSubscription,
} from "@/Services/subscriptions/subscriptionsService";
import { QUERY_KEYS } from "@/constants/queryKeys";

export function useMySubscriptions() {
  return useQuery({
    queryKey: QUERY_KEYS.subscriptions.mine(),
    queryFn: listMySubscriptions,
    staleTime: 1000 * 60 * 5,
  });
}

export function useActiveSubscription() {
  return useQuery({
    queryKey: QUERY_KEYS.subscriptions.active(),
    queryFn: getMyActiveSubscription,
    staleTime: 1000 * 60 * 5,
  });
}
