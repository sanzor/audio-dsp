import { useEffect } from "react";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { apiGetCompileTicketStatus } from "@/Services/TicketService";
import { QUERY_KEYS } from "@/constants/queryKeys";

// Polls a compile ticket until it reaches a terminal state (successful/failed).
// On success, invalidates the transform's cached binary and definition so the
// newly compiled artifact gets refetched.
export function useCompileTicketStatus(ticketId: number | null, transformId: number | null) {
  const qc = useQueryClient();
  const query = useQuery({
    queryKey: QUERY_KEYS.tickets.byId(ticketId ?? -1),
    queryFn: () => apiGetCompileTicketStatus(ticketId!),
    enabled: ticketId != null,
    refetchInterval: (q) => (q.state.data?.status.state === "processing" ? 1500 : false),
  });

  useEffect(() => {
    if (query.data?.status.state === "successful" && transformId != null) {
      qc.invalidateQueries({ queryKey: ["transform", "wasm"] });
      qc.invalidateQueries({ queryKey: QUERY_KEYS.transforms.byId(transformId) });
    }
  }, [query.data?.status.state, transformId, qc]);

  return query;
}
