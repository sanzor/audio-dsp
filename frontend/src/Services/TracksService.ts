import type { TrackMeta } from '@/domain/Track/TrackMeta';
import type { TrackInfo } from '@/domain/Track/TrackInfo';
import type { ABuffer } from '@/domain/ABuffer';
import { http, API_BASE_URL } from '@/Services/http';
import { useAuthStore } from '@/Stores/authStore';
import { useProjectStore } from '@/Stores/projectStore';

// ─── Params & Results ────────────────────────────────────────────────────────

export interface RawTrack {
  info: TrackInfo;
  data: ABuffer;
}

export interface GetTrackParams {
  track_id: number;
}
export interface GetTrackResult {
  track: TrackMeta;
}

export interface GetTrackRawParams {
  track_id: number;
}
export interface GetTrackRawResult {
  track_id: number;
  blob: Blob;
}

export interface CreateTrackParams {
  rawTrack: RawTrack;
  fileBlob?: Blob;
}
export interface CreateTrackResult {
  track_id: number;
  track_info: TrackInfo;
}

export interface CopyTrackParams {
  track_id: number;
  copy_track_name: string;
}
export interface CopyTrackResult {
  track: TrackMeta;
}

export interface UpdateTrackParams {
  track_id: number;
  track_name: string;
}
export interface UpdateTrackResult {
  track_id: number;
  name: string;
}

export interface RemoveTrackParams {
  trackId: number;
}
export interface RemoveTrackResult {
  id: number;
}

// ─── API ──────────────────────────────────────────────────────────────────────

export async function apiGetTracks(): Promise<TrackMeta[]> {
  const json: Array<{ track_id: number; track_info: TrackInfo }> = await http.get('/tracks/get-all');
  return json.map((t) => ({ trackId: t.track_id, trackInfo: t.track_info, regionSets: [] }));
}

export function apiGetTrackMeta(params: GetTrackParams): Promise<GetTrackResult> {
  return http.get(`/tracks/get-meta?track_id=${params.track_id}`);
}

export async function apiGetTrackInfo(params: GetTrackParams): Promise<GetTrackResult> {
  const t: { track_id: number; track_info: TrackInfo } = await http.get(`/tracks/get-track-info?track_id=${params.track_id}`);
  return { track: { trackId: t.track_id, trackInfo: t.track_info, regionSets: [] } };
}

export async function apiGetStoredTrack(params: GetTrackRawParams): Promise<GetTrackRawResult> {
  const token = useAuthStore.getState().token;
  const activeProjectId = useProjectStore.getState().activeProject?.project_id;

  const res = await fetch(`${API_BASE_URL}/stored-tracks/get?track_id=${params.track_id}`, {
    headers: {
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
      ...(activeProjectId != null ? { "X-Project-Id": String(activeProjectId) } : {}),
    },
  });

  if (!res.ok) throw new Error(`Failed to fetch track: ${res.status} ${res.statusText}`);

  const blob = await res.blob();
  if (blob.size === 0) throw new Error('Received empty audio data');

  const audioBlob = blob.type?.startsWith('audio/') ? blob : new Blob([blob], { type: 'audio/wav' });
  return { blob: audioBlob, track_id: params.track_id };
}

export async function apiAddTrack(params: CreateTrackParams): Promise<CreateTrackResult> {
  const token = useAuthStore.getState().token;
  const activeProjectId = useProjectStore.getState().activeProject?.project_id;

  const formData = new FormData();
  formData.append("name", params.rawTrack.info.name);

  if (params.fileBlob) {
    formData.append("extension", params.rawTrack.info.extension ?? "wav");
    formData.append("samples", params.fileBlob, "samples.wav");
  } else {
    formData.append("extension", params.rawTrack.info.extension ?? "wav");
    formData.append("sample_rate", String(params.rawTrack.data.sample_rate));
    formData.append("channels", String(params.rawTrack.data.channels));
    const blob = new Blob([new Float32Array(params.rawTrack.data.samples).buffer], { type: "application/octet-stream" });
    formData.append("samples", blob, "samples.raw");
  }

  const res = await fetch(`${API_BASE_URL}/tracks/add-track-multi`, {
    method: "POST",
    headers: {
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
      ...(activeProjectId != null ? { "X-Project-Id": String(activeProjectId) } : {}),
    },
    body: formData,
  });

  if (!res.ok) throw new Error(`Upload failed: ${res.statusText}`);
  return await res.json();
}

export function apiRemoveTrack(params: RemoveTrackParams): Promise<RemoveTrackResult> {
  return http.delete(`/tracks/remove?track_id=${params.trackId}`);
}

export function apiUpdateTrack(params: UpdateTrackParams): Promise<UpdateTrackResult> {
  return http.post("/tracks/update-track-info", params);
}

export function apiCopyTrack(params: CopyTrackParams): Promise<CopyTrackResult> {
  return http.post("/tracks/copy-track", params);
}
