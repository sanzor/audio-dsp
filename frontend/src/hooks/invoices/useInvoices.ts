import { useQuery } from "@tanstack/react-query";
import { listMyInvoices } from "@/Services/invoices/invoicesService";
import { QUERY_KEYS } from "@/constants/queryKeys";

export function useMyInvoices(offset = 0) {
  return useQuery({
    queryKey: QUERY_KEYS.invoices.mine(offset),
    queryFn: () => listMyInvoices(offset),
    staleTime: 1000 * 60 * 5,
  });
}
