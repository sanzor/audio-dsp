import { useQuery } from "@tanstack/react-query";
import { listTierConfigs } from "@/Services/tierConfigs/tierConfigsService";
import { QUERY_KEYS } from "@/constants/queryKeys";

export function useTierConfigs() {
  return useQuery({
    queryKey: QUERY_KEYS.tierConfigs.all(),
    queryFn: listTierConfigs,
    staleTime: 1000 * 60 * 10,
  });
}
