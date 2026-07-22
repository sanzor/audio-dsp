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
  // Present only once state === "successful" — the compiled binary,
  // base64-encoded, so the creator surface can run a "Try it" preview
  // before deciding to save. Never sent back to the backend. See
  // agents/decisions/0003-transform-preview-flow.md.
  wasm_base64?: string;
  // Present alongside wasm_base64 — lets "Try it" seed the preview with
  // this build's real declared param defaults instead of zeros.
  params?: CompileParam[];
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
  wasm_base64: string;
  params: CompileParam[];
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
