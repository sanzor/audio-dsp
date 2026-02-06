// /auth/auth-service.ts

import type { GetTrackParams } from '@/Dtos/Tracks/GetTrackParams';

import type { GetTrackResult } from '@/Dtos/Tracks/GetTrackResult';
import type { UpdateTrackParams } from '@/Dtos/Tracks/UpdateTrackParams';
import type { UpdateTrackResult } from '@/Dtos/Tracks/UpdateTrackResult';
import type { TrackMeta } from '@/Domain/Track/TrackMeta';
import type { CopyTrackParams } from '@/Dtos/Tracks/CopyTrackParams';
import type { CopyTrackResult } from '@/Dtos/Tracks/CopyTrackResult';
import type { GetTrackRawParams } from '@/Dtos/Tracks/GetTrackRawParams';
import type { GetTrackRawResult } from '@/Dtos/Tracks/GetTrackRawResult';
import type { CreateTrackParams } from '@/Dtos/Tracks/AddTrackParams';
import type { CreateTrackResult } from '@/Dtos/Tracks/AddTrackResult';
import type { RemoveTrackParams } from '@/Dtos/Tracks/RemoveTrackParams';
import type { RemoveTrackResult } from '@/Dtos/Tracks/RemoveTrackResult';

const BASE_URL = import.meta.env.VITE_API_BASE_URL;

/**
 * Fetches the current user session.
 */
export async function apiGetTracks(): Promise<TrackMeta[]> {
  const res = await fetch(`${BASE_URL}/tracks/get-all`, {
    method: 'GET',
    credentials: 'include',
  });

  if (!res.ok) throw new Error('Failed to fetch session');

  const json = await res.json(); // ✅ await here
  console.log(json);
  return json.tracks;
}

export async function apiGetTrackMeta(params:GetTrackParams): Promise<GetTrackResult> {
  const res = await fetch(`${BASE_URL}/tracks/get-meta?track_id=${params.track_id}`, {
    method: 'GET',
    credentials: 'include'
  });

  if (!res.ok) throw new Error('Refresh token failed');
  return await res.json();
}

export async function apiGetTrackInfo(params:GetTrackParams): Promise<GetTrackResult> {
  const res = await fetch(`${BASE_URL}/tracks/get-track-info?track_id=${params.track_id}`, {
    method: 'GET',
    credentials: 'include'
  });

  if (!res.ok) throw new Error('Refresh token failed');
  return await res.json();
}

export async function apiGetStoredTrack(params: GetTrackRawParams): Promise<GetTrackRawResult> {
  const res = await fetch(`${BASE_URL}/tracks/get-stored-track?track_id=${params.track_id}`, {
    method: 'GET',
    credentials: 'include'
  });

  if (!res.ok) {
    const errorText = await res.text();
    console.error('API Error:', {
      status: res.status,
      statusText: res.statusText,
      body: errorText
    });
    throw new Error(`Failed to fetch track: ${res.status} ${res.statusText}`);
  }

  const blob = await res.blob();
  
  // Debug the blob we received
  console.log('Received blob:', {
    size: blob.size,
    type: blob.type,
    hasContent: blob.size > 0
  });

  // Verify it's actually audio data
  if (blob.size === 0) {
    throw new Error('Received empty audio data');
  }

  // If the blob doesn't have the correct MIME type, fix it
  let audioBlob = blob;
  if (!blob.type || !blob.type.startsWith('audio/')) {
    console.log('Fixing blob MIME type from', blob.type, 'to audio/wav');
    audioBlob = new Blob([blob], { type: 'audio/wav' });
  }
  console.log("🧪 Blob type:", blob.type); // should be 'audio/mpeg' or 'audio/wav'
  console.log("🧪 Blob size:", blob.size);
  return {
    blob: audioBlob,
    track_id: params.track_id
  };
}

export async function apiAddTrack(params: CreateTrackParams): Promise<CreateTrackResult> {
  console.log("Inside api add track");
  const formData = new FormData();

  formData.append("name", params.rawTrack.info.name);
  formData.append("extension", params.rawTrack.info.extension ?? "wav");

  // ABuffer metadata
  formData.append("sample_rate", String(params.rawTrack.data.sample_rate));
  formData.append("channels", String(params.rawTrack.data.channels));

  // Convert samples to Blob
  const blob = new Blob([new Float32Array(params.rawTrack.data.samples).buffer], {
    type: "application/octet-stream"
  });
  formData.append("samples", blob, "samples.raw");

  const res = await fetch(`${BASE_URL}/tracks/add-track-multi`, {
    method: "POST",
    credentials: "include",
    body: formData,
  });

  if (!res.ok) throw new Error(`Upload failed: ${res.statusText}`);
  return await res.json();
}

export async function apiRemoveTrack(params:RemoveTrackParams): Promise<RemoveTrackResult> {
  const res = await fetch(`${BASE_URL}/tracks/remove?track_id=${params.trackId}`, {
    method: 'DELETE',
    credentials: 'include'
  });

  if (!res.ok) throw new Error('Refresh token failed');
  return await res.json();
}

export async function apiUpdateTrack(params: UpdateTrackParams): Promise<UpdateTrackResult> {
  console.log("a");
  const res = await fetch(`${BASE_URL}/tracks/update-track-info`, {
    method: 'POST', // ✅ must be POST to send body
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(params),
  });
  console.log(res.status);
  if (!res.ok) {
    throw new Error(`Failed to update track: ${res.statusText}`);
  }
  return await res.json();
}

export async function apiCopyTrack(params: CopyTrackParams): Promise<CopyTrackResult> {
  const res = await fetch(`${BASE_URL}/tracks/copy-track`, {
    method: 'POST', // ✅ must be POST to send body
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(params),
  });

  if (!res.ok) {
    throw new Error(`Failed to update track: ${res.statusText}`);
  }
  return await res.json();
}

