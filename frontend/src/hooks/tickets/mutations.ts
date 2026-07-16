import { useMutation } from "@tanstack/react-query";
import { apiCreateCompileTicket, type CreateCompileTicketParams } from "@/Services/TicketService";

export function useRequestCompileTransform() {
  return useMutation({
    mutationFn: (params: CreateCompileTicketParams) => apiCreateCompileTicket(params),
  });
}
