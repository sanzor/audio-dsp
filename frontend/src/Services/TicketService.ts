import { http } from "@/Services/http";

// ─── Params ──────────────────────────────────────────────────────────────────

export interface CreateCompileTicketParams {
  transform_id: number;
  source_code: string;
}

export type CompileTicketState = "processing" | "failed" | "successful";

export interface CompileParam {
  name: string;
  param_order: number;
  default_value: number;
  min_value?: number;
  max_value?: number;
  description?: string;
}

export interface CompileTicketStatus {
  state: CompileTicketState;
  resource_id?: number;
  message?: string;
}

export interface CompileTicket {
  ticket_id: number;
  issued_by: number;
  status: CompileTicketStatus;
  timestamp: number;
}

export interface CompileResource {
  resource_id: number;
  ticket_id: number;
  source_code: string;
  wasm_base64: string;
}

// ─── API ─────────────────────────────────────────────────────────────────────

export async function apiCreateCompileTicket(
  params: CreateCompileTicketParams
): Promise<CompileTicket> {
  return http.post<CompileTicket, CreateCompileTicketParams>(`/transforms/tickets`, params);
}

export async function apiGetCompileTicketStatus(ticket_id: number): Promise<CompileTicket> {
  return http.get<CompileTicket>(`/transforms/tickets/${ticket_id}`);
}

export async function apiGetCompileResource(resource_id: number): Promise<CompileResource> {
  return http.get<CompileResource>(`/transforms/resources/${resource_id}`);
}
