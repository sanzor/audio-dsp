import type { TrackRegionSet } from "@/domain/RegionSet/TrackRegionSet";
import { authFetch } from "@/Services/http";

const BASE_URL = import.meta.env.VITE_API_BASE_URL;

// ─── Params & Results ────────────────────────────────────────────────────────

export interface GetRegionSetParams {
  region_set_id: string;
}
export interface GetRegionSetResult {
  region_set: TrackRegionSet;
}

export interface GetRegionSetsForTrackResult {
  trackId: string;
  sets: TrackRegionSet[];
}

export interface GetRegionSetsResult {
  track_region_sets_map: Map<string, TrackRegionSet[]>;
}

export interface CreateRegionSetParams {
  name: string | null;
  track_id: string;
}
export interface CreateRegionSetResult {
  region_set: TrackRegionSet;
}

export interface CopyRegionSetParams {
  sourceRegionSetId: string;
  destTrackId: string;
  copy_region_set_name: string;
}

export interface EditRegionSetParams {
  region_set_id: string;
  trackId: string;
  name: string | null;
}
export interface EditRegionSetResult {
  region_set: TrackRegionSet;
}

export interface RemoveRegionSetParams {
  regionSetId: string;
}
export interface RemoveRegionSetResult {}

// ─── API ──────────────────────────────────────────────────────────────────────

export async function apiGetRegionSet(regionSetId: string): Promise<TrackRegionSet> {
  const res = await authFetch(`${BASE_URL}/region-sets/get-region-set?region_set_id=${regionSetId}`, {
    method: 'GET',
  });
  if (!res.ok) throw new Error('Failed to fetch session');
  const json = await res.json();
  console.log(json);
  return json.tracks;
}

export async function apiGetRegionSetsForTrack(trackId: string): Promise<GetRegionSetsForTrackResult> {
  const res = await authFetch(`${BASE_URL}/region-sets/get-all-for-track?track_id=${trackId}`, {
    method: 'GET',
  });
  if (!res.ok) throw new Error('Failed to fetch session');
  const json = await res.json();
  console.log(json);
  return json.tracks;
}

export async function apiGetAllRegionSets(): Promise<GetRegionSetsResult> {
  const res = await authFetch(`${BASE_URL}/region-sets/get-region-sets`, {
    method: 'GET',
  });
  if (!res.ok) throw new Error('Failed to fetch session');
  const json = await res.json();
  console.log(json);
  return json.tracks;
}

export async function apiCreateRegionSet(params: CreateRegionSetParams): Promise<CreateRegionSetResult> {
  console.log("a");
  const res = await authFetch(`${BASE_URL}/region-sets/create`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(params),
  });
  console.log(res.status);
  if (!res.ok) throw new Error(`Failed to update track: ${res.statusText}`);
  return await res.json();
}

export async function apiUpdateRegionSet(params: EditRegionSetParams): Promise<EditRegionSetResult> {
  console.log("a");
  const res = await authFetch(`${BASE_URL}/region-sets/edit`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(params),
  });
  console.log(res.status);
  if (!res.ok) throw new Error(`Failed to update track: ${res.statusText}`);
  return await res.json();
}

export async function apiRemoveRegionSet(params: RemoveRegionSetParams): Promise<void> {
  const res = await authFetch(`${BASE_URL}/region-sets/remove?region_set_id=${params.regionSetId}`, {
    method: 'DELETE',
  });
  if (!res.ok) throw new Error('Refresh token failed');
}

export async function apiCopyRegionSet(params: CopyRegionSetParams): Promise<CreateRegionSetResult> {
  console.log("a");
  const res = await authFetch(`${BASE_URL}/region-sets/create`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(params),
  });
  console.log(res.status);
  if (!res.ok) throw new Error(`Failed to update track: ${res.statusText}`);
  return await res.json();
}
