import { http, API_BASE_URL } from "@/Services/http";
import { useAuthStore } from "@/Stores/authStore";
import { useProjectStore } from "@/Stores/projectStore";

// Creator-surface "signal sources" — real uploaded audio files selectable in
// the shared "Try it" preview session, as an alternative to the synthetic
// 440Hz test tone. See backend/domain/src/sources/source_info.rs. Not to be
// confused with the "source"/"sink" nodeType values inside Editor-side
// graph_state JSONB — unrelated.
//
// Known backend gap (accepted, not fixed client-side): none of these
// endpoints scope by project — list-sources returns sources from every
// project, and the others don't verify the source belongs to the caller's
// project.

// ─── Types ───────────────────────────────────────────────────────────────────
// Mirrors backend/domain/src/sources/source_info.rs.
export interface SourceInfo {
  name: string;
  extension: string;
  length: number; // seconds
}

// Mirrors backend/domain/src/sources/source_meta.rs -- field names are
// source_info/source_id, not flattened.
export interface SourceMeta {
  source_info: SourceInfo;
  source_id: number;
}

export interface AddSourceMultiParams {
  name: string;
  extension: string;
  fileBlob: Blob;
}

// Mirrors AddSourceResult in backend/api/src/controllers/sources_controller.rs.
export interface AddSourceResult {
  source_id: number;
  source_info: SourceInfo;
}

export interface RenameSourceParams {
  source_id: number;
  source_name: string;
}

// ─── API ─────────────────────────────────────────────────────────────────────

export function apiListSources(): Promise<SourceMeta[]> {
  return http.get<SourceMeta[]>("/sources/list-sources");
}

// Uploads WAV bytes as `samples` -- the backend sniffs WAV via a RIFF/WAVE
// header; it never decodes compressed formats server-side. Callers
// (sources-panel-modal.tsx) are expected to have already decoded whatever
// the user picked via the Web Audio API and re-encoded it as WAV before
// calling this, so `params.fileBlob` here is always WAV regardless of the
// source file's original format.
export async function apiAddSourceMulti(params: AddSourceMultiParams): Promise<AddSourceResult> {
  const token = useAuthStore.getState().token;
  const activeProjectId = useProjectStore.getState().activeProject?.project_id;

  const formData = new FormData();
  formData.append("name", params.name);
  formData.append("extension", params.extension);
  formData.append("samples", params.fileBlob, `samples.${params.extension}`);

  const res = await fetch(`${API_BASE_URL}/sources/add-source-multi`, {
    method: "POST",
    headers: {
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
      ...(activeProjectId != null ? { "X-Project-Id": String(activeProjectId) } : {}),
    },
    body: formData,
  });

  if (!res.ok) throw new Error(`Source upload failed: ${res.status} ${res.statusText}`);
  return await res.json();
}

export function apiRenameSource(params: RenameSourceParams): Promise<SourceMeta> {
  return http.post<SourceMeta, RenameSourceParams>("/sources/rename-source", params);
}

export async function apiDeleteSource(sourceId: number): Promise<void> {
  await http.delete<unknown>(`/sources/delete-source?source_id=${sourceId}`);
}

// Fetches a source's raw audio bytes -- mirrors apiGetStoredTrack's
// /stored-tracks/get pattern (TracksService.ts). Used both for inline
// audition in the sources management panel and to feed the "Try it" preview
// session's input when a source is picked in place of the synthetic tone.
export async function apiGetSourceAudio(sourceId: number): Promise<Blob> {
  const token = useAuthStore.getState().token;
  const activeProjectId = useProjectStore.getState().activeProject?.project_id;

  const res = await fetch(`${API_BASE_URL}/sources/get-audio?source_id=${sourceId}`, {
    headers: {
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
      ...(activeProjectId != null ? { "X-Project-Id": String(activeProjectId) } : {}),
    },
  });

  if (!res.ok) throw new Error(`Failed to fetch source audio: ${res.status} ${res.statusText}`);
  const blob = await res.blob();
  if (blob.size === 0) throw new Error("Received empty audio data");
  return blob;
}
