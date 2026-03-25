import type { TrackRegion } from "@/domain/Region/TrackRegion";
import { authFetch } from "@/Services/http";

const BASE_URL = import.meta.env.VITE_API_BASE_URL;

// ─── Params & Results ────────────────────────────────────────────────────────

export interface GetRegionsForRegionSetResult {
  regions: TrackRegion[];
}

export interface CreateRegionParams {
  start_time: number;
  end_time: number | null;
  name: string;
  region_set_id: string;
}
export interface CreateRegionResult {
  region: TrackRegion;
}

export interface CopyRegionParams {
  sourceRegionId: string;
  destinationRegionSetId: string;
  copyName: string;
}
export interface CopyRegionResult {
  region: TrackRegion;
}

export interface EditRegionParams {
  regionId: string;
  name: string;
}
export interface EditRegionResult {
  region: TrackRegion;
}

export interface RemoveRegionParams {
  regionId: string;
}
export interface RemoveRegionResult {
  result: string;
}

// ─── API ──────────────────────────────────────────────────────────────────────

export async function apiGetRegion(regionId: string): Promise<TrackRegion> {
  const res = await authFetch(`${BASE_URL}/region-sets/get-region?region_set_id=${regionId}`, {
    method: 'GET',
  });
  if (!res.ok) throw new Error('Failed to fetch region');
  const json = await res.json();
  console.log(json);
  return json.tracks;
}

export async function apiGetRegionsForRegionSet(regionSetId: string): Promise<GetRegionsForRegionSetResult> {
  const res = await authFetch(`${BASE_URL}/regions/get-regions?region-set-id=${regionSetId}`, {
    method: 'GET',
  });
  if (!res.ok) throw new Error('Failed to fetch region');
  const json = await res.json();
  console.log(json);
  return json.tracks;
}

export async function apiAddRegion(params: CreateRegionParams): Promise<CreateRegionResult> {
  console.log("a");
  const res = await authFetch(`${BASE_URL}/regions/add`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(params),
  });
  console.log(res.status);
  if (!res.ok) throw new Error(`Failed to update track: ${res.statusText}`);
  return await res.json();
}

export async function apiEditRegion(params: EditRegionParams): Promise<EditRegionResult> {
  console.log("a");
  const res = await authFetch(`${BASE_URL}/regions/edit`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(params),
  });
  console.log(res.status);
  if (!res.ok) throw new Error(`Failed to update region: ${res.statusText}`);
  return await res.json();
}

export async function apiRemoveRegion(params: RemoveRegionParams): Promise<RemoveRegionResult> {
  const res = await authFetch(`${BASE_URL}/regions/remove?track_id=${params.regionId}`, {
    method: 'DELETE',
  });
  if (!res.ok) throw new Error('Failed to remove region');
  return await res.json();
}

export async function apiCopyRegion(params: CopyRegionParams): Promise<CopyRegionResult> {
  const res = await authFetch(`${BASE_URL}/regions/copy`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(params),
  });
  console.log(res.status);
  if (!res.ok) throw new Error(`Failed to update region: ${res.statusText}`);
  return await res.json();
}
