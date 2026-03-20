import { useQuery } from "@tanstack/react-query";
import { listProducts } from "@/Services/products/productsService";
import { QUERY_KEYS } from "@/constants/queryKeys";

export function useProducts(activeOnly = true) {
  return useQuery({
    queryKey: activeOnly ? QUERY_KEYS.products.active() : QUERY_KEYS.products.all(),
    queryFn: () => listProducts(activeOnly),
    staleTime: 1000 * 60 * 5,
  });
}
