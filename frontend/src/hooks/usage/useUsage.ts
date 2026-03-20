import { useQuery } from "@tanstack/react-query";
import { getMyUsage } from "@/Services/usage/usageService";
import { QUERY_KEYS } from "@/constants/queryKeys";

export function useMyUsage() {
  return useQuery({
    queryKey: QUERY_KEYS.usage.mine(),
    queryFn: getMyUsage,
    staleTime: 1000 * 30,
    retry: false,
  });
}
