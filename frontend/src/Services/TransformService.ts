import type { TransformDefinition, TransformPort, TransformSummary } from "@/domain/Transform/Transform";
import { http, API_BASE_URL } from "@/Services/http";
import { useAuthStore } from "@/Stores/authStore";
import { useProjectStore } from "@/Stores/projectStore";

// ─── Params ──────────────────────────────────────────────────────────────────

export interface CreateTransformParams {
  name: string;
  description?: string;
  icon?: string;
}

export interface UpdateTransformParams {
  name: string;
  description?: string;
  icon?: string;
}

export interface AddPortParams {
  name: string;
  direction: "input" | "output";
  port_order: number;
  description?: string;
}

export interface CreateCompileTicketParams {
  transform_id: number;
  source_code: string;
}

export type CompileTicketState = "processing" | "failed" | "successful";

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
}

// ─── API ─────────────────────────────────────────────────────────────────────

export interface TransformPage {
  transforms: TransformSummary[];
  total: number;
}

export interface TransformDefinitionsResponse {
  transforms: TransformDefinition[];
}

export interface TransformIdsRequest {
  ids: number[];
}

export interface TransformBinaryEnvelope {
  transform_id: number;
  wasm_base64: string;
}

export interface TransformBinariesResponse {
  binaries: TransformBinaryEnvelope[];
}

function decodeBase64Binary(encoded: string): Uint8Array {
  const decoded = atob(encoded);
  const bytes = new Uint8Array(decoded.length);
  for (let i = 0; i < decoded.length; i += 1) {
    bytes[i] = decoded.charCodeAt(i);
  }
  return bytes;
}

export function mapTransformBinariesResponse(response: TransformBinariesResponse): Map<number, Uint8Array> {
  return new Map(
    response.binaries.map((binary) => [
      binary.transform_id,
      decodeBase64Binary(binary.wasm_base64),
    ])
  );
}

export async function apiGetTransformSummaries(offset = 0, limit = 20): Promise<TransformPage> {
  return http.get<TransformPage>(`/transforms?offset=${offset}&limit=${limit}`);
}

export async function apiGetTransformDefinition(transform_id: number): Promise<TransformDefinition> {
  return http.get<TransformDefinition>(`/transforms/${transform_id}`);
}

export async function apiResolveTransformDefinitions(transform_ids: number[]): Promise<TransformDefinition[]> {
  const response = await http.post<TransformDefinitionsResponse, TransformIdsRequest>(
    `/transforms/resolve`,
    { ids: transform_ids }
  );
  return response.transforms;
}

export async function apiCreateTransform(params: CreateTransformParams): Promise<TransformDefinition> {
  return http.post<TransformDefinition, CreateTransformParams>(`/transforms`, params);
}

export async function apiUpdateTransform(
  transform_id: number,
  params: UpdateTransformParams
): Promise<TransformDefinition> {
  return http.put<TransformDefinition, UpdateTransformParams>(
    `/transforms/${transform_id}`,
    params
  );
}

export async function apiDeleteTransform(transform_id: number): Promise<void> {
  await http.delete<void>(`/transforms/${transform_id}`);
}

export async function apiAddPort(
  transform_id: number,
  params: AddPortParams
): Promise<TransformPort> {
  return http.post<TransformPort, AddPortParams>(
    `/transforms/${transform_id}/ports`,
    params
  );
}

export async function apiDeletePort(port_id: number): Promise<void> {
  await http.delete<void>(`/transforms/ports/${port_id}`);
}

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

// Fetches the pre-compiled .wasm binary for a transform from the backend.
// The backend reads the committed binary bytes from persisted storage; the frontend never compiles.
export async function apiGetTransformBinary(transform_id: number): Promise<Uint8Array> {
  const token = useAuthStore.getState().token ?? undefined;
  const activeProjectId = useProjectStore.getState().activeProject?.project_id;
  const response = await fetch(`${API_BASE_URL}/transforms/${transform_id}/binary`, {
    headers: {
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
      ...(activeProjectId != null ? { "X-Project-Id": String(activeProjectId) } : {}),
    },
  });
  if (!response.ok) throw new Error(`Failed to fetch wasm for transform ${transform_id}`);
  const buffer = await response.arrayBuffer();
  return new Uint8Array(buffer);
}

export async function apiFetchTransformBinaries(request: TransformIdsRequest): Promise<TransformBinariesResponse> {
  return http.post<TransformBinariesResponse, TransformIdsRequest>(
    `/transforms/binaries`,
    request
  );
}

export async function apiGetTransformBinaries(transform_ids: number[]): Promise<Map<number, Uint8Array>> {
  const response = await apiFetchTransformBinaries({ ids: transform_ids });
  return mapTransformBinariesResponse(response);
}
