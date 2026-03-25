import type { TrackMeta } from '@/domain/Track/TrackMeta';
import type { TrackInfo } from '@/domain/Track/TrackInfo';
import type { ABuffer } from '@/domain/ABuffer';
import { authFetch } from '@/Services/http';

const BASE_URL = import.meta.env.VITE_API_BASE_URL;

function buildTrackHeaders(headers?: HeadersInit): Headers {
  return new Headers(headers);
}

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
  const res = await authFetch(`${BASE_URL}/tracks/get-all`, {
    method: 'GET',
    headers: buildTrackHeaders(),
  });
  if (!res.ok) throw new Error('Failed to fetch tracks');
  const json: Array<{ track_id: number; track_info: TrackInfo }> = await res.json();
  return json.map((t) => ({
    trackId: t.track_id,
    trackInfo: t.track_info,
    regionSets: [],
  }));
}

export async function apiGetTrackMeta(params: GetTrackParams): Promise<GetTrackResult> {
  const res = await authFetch(`${BASE_URL}/tracks/get-meta?track_id=${params.track_id}`, {
    method: 'GET',
    headers: buildTrackHeaders(),
  });
  if (!res.ok) throw new Error('Refresh token failed');
  return await res.json();
}

export async function apiGetTrackInfo(params: GetTrackParams): Promise<GetTrackResult> {
  const res = await authFetch(`${BASE_URL}/tracks/get-track-info?track_id=${params.track_id}`, {
    method: 'GET',
    headers: buildTrackHeaders(),
  });
  if (!res.ok) throw new Error('Refresh token failed');
  const t: { track_id: number; track_info: TrackInfo } = await res.json();
  return { track: { trackId:t.track_id, trackInfo: t.track_info, regionSets: [] } };
}

export async function apiGetStoredTrack(params: GetTrackRawParams): Promise<GetTrackRawResult> {
  const res = await authFetch(`${BASE_URL}/tracks/get-stored-track?track_id=${params.track_id}`, {
    method: 'GET',
    headers: buildTrackHeaders(),
  });

  if (!res.ok) {
    const errorText = await res.text();
    console.error('API Error:', { status: res.status, statusText: res.statusText, body: errorText });
    throw new Error(`Failed to fetch track: ${res.status} ${res.statusText}`);
  }

  const blob = await res.blob();
  console.log('Received blob:', { size: blob.size, type: blob.type, hasContent: blob.size > 0 });

  if (blob.size === 0) throw new Error('Received empty audio data');

  let audioBlob = blob;
  if (!blob.type || !blob.type.startsWith('audio/')) {
    console.log('Fixing blob MIME type from', blob.type, 'to audio/wav');
    audioBlob = new Blob([blob], { type: 'audio/wav' });
  }
  console.log("🧪 Blob type:", blob.type);
  console.log("🧪 Blob size:", blob.size);
  return { blob: audioBlob, track_id: params.track_id as number };
}

export async function apiAddTrack(params: CreateTrackParams): Promise<CreateTrackResult> {
  console.log("Inside api add track");
  const formData = new FormData();
  formData.append("name", params.rawTrack.info.name);
  const uploadBlob = params.fileBlob;

  if (uploadBlob) {
    formData.append("extension", params.rawTrack.info.extension ?? "wav");
    formData.append("samples", uploadBlob, "samples.wav");
  } else {
    formData.append("extension", params.rawTrack.info.extension ?? "wav");
    formData.append("sample_rate", String(params.rawTrack.data.sample_rate));
    formData.append("channels", String(params.rawTrack.data.channels));
    const blob = new Blob([new Float32Array(params.rawTrack.data.samples).buffer], {
      type: "application/octet-stream",
    });
    formData.append("samples", blob, "samples.raw");
  }

  const res = await authFetch(`${BASE_URL}/tracks/add-track-multi`, {
    method: "POST",
    headers: buildTrackHeaders(),
    body: formData,
  });
  if (!res.ok) throw new Error(`Upload failed: ${res.statusText}`);
  return await res.json();
}

export async function apiRemoveTrack(params: RemoveTrackParams): Promise<RemoveTrackResult> {
  const res = await authFetch(`${BASE_URL}/tracks/remove?track_id=${params.trackId}`, {
    method: 'DELETE',
    headers: buildTrackHeaders(),
  });
  if (!res.ok) throw new Error('Refresh token failed');
  return await res.json();
}

export async function apiUpdateTrack(params: UpdateTrackParams): Promise<UpdateTrackResult> {
  console.log("a");
  const res = await authFetch(`${BASE_URL}/tracks/update-track-info`, {
    method: 'POST',
    headers: buildTrackHeaders({ 'Content-Type': 'application/json' }),
    body: JSON.stringify(params),
  });
  console.log(res.status);
  if (!res.ok) throw new Error(`Failed to update track: ${res.statusText}`);
  return await res.json();
}

export async function apiCopyTrack(params: CopyTrackParams): Promise<CopyTrackResult> {
  const res = await authFetch(`${BASE_URL}/tracks/copy-track`, {
    method: 'POST',
    headers: buildTrackHeaders({ 'Content-Type': 'application/json' }),
    body: JSON.stringify(params),
  });
  if (!res.ok) throw new Error(`Failed to update track: ${res.statusText}`);
  return await res.json();
}
