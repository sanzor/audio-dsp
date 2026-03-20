import { useQuery } from "@tanstack/react-query";
import { listMyPurchases } from "@/Services/purchasedProducts/purchasedProductsService";
import { QUERY_KEYS } from "@/constants/queryKeys";

export function usePurchases() {
  return useQuery({
    queryKey: QUERY_KEYS.purchases.mine(),
    queryFn: listMyPurchases,
    staleTime: 1000 * 60 * 5,
  });
}
