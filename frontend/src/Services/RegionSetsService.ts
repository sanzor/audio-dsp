import type { TrackRegionSet } from "@/domain/RegionSet/TrackRegionSet";
import type { TrackRegion } from "@/domain/Region/TrackRegion";
import { http } from "@/Services/http";

interface ApiRegion {
  regionId: number;
  regionSetId: number;
  name: string;
  startTime: number;
  endTime: number;
  graph?: TrackRegion["graph"];
}

interface ApiRegionSet {
  id: number;
  trackId: number;
  name: string;
  regions?: ApiRegion[];
}

const mapApiRegion = (region: ApiRegion): TrackRegion => ({
  regionId: region.regionId,
  regionSetId: region.regionSetId,
  name: region.name,
  start: region.startTime,
  end: region.endTime,
  graph: region.graph,
});

const mapApiRegionSet = (regionSet: ApiRegionSet): TrackRegionSet => ({
  id: regionSet.id,
  trackId: regionSet.trackId,
  name: regionSet.name,
  regions: (regionSet.regions ?? []).map(mapApiRegion),
});

// ─── Params & Results ────────────────────────────────────────────────────────

export interface GetRegionSetParams {
  region_set_id: number;
}
export interface GetRegionSetResult {
  region_set: TrackRegionSet;
}

export interface GetRegionSetsForTrackResult {
  trackId: number;
  regionSets: TrackRegionSet[];
}

export interface GetRegionSetsResult {
  track_region_sets_map: Record<number, TrackRegionSet[]>;
}

export interface CreateRegionSetParams {
  name: string | null;
  trackId: number;
}
export type CreateRegionSetResult = TrackRegionSet;

export interface CopyRegionSetParams {
  regionSetId: number;
  copyName: string;
}

export interface EditRegionSetParams {
  region_set_id: number;
  trackId: number;
  name: string | null;
}
export interface EditRegionSetResult {
  region_set: TrackRegionSet;
}

export interface RemoveRegionSetParams {
  regionSetId: number;
}
export interface RemoveRegionSetResult {}

// ─── API ──────────────────────────────────────────────────────────────────────

export async function apiGetRegionSet(regionSetId: number): Promise<TrackRegionSet> {
  const response = await http.get<ApiRegionSet>(`/region-sets/get?region_set_id=${regionSetId}`);
  return mapApiRegionSet(response);
}

export async function apiGetRegionSetsForTrack(trackId: number): Promise<GetRegionSetsForTrackResult> {
  const response = await http.get<{ trackId: number; regionSets: ApiRegionSet[] }>(`/region-sets/get-all-for-track?trackId=${trackId}`);
  return { trackId: response.trackId, regionSets: response.regionSets.map(mapApiRegionSet) };
}

export async function apiGetAllRegionSets(): Promise<GetRegionSetsResult> {
  const response = await http.get<{ track_region_sets_map: Record<string, ApiRegionSet[]> }>("/region-sets/get-all");
  const mapped: Record<number, TrackRegionSet[]> = {};
  for (const [trackId, sets] of Object.entries(response.track_region_sets_map)) {
    mapped[Number(trackId)] = sets.map(mapApiRegionSet);
  }
  return { track_region_sets_map: mapped };
}

export async function apiCreateRegionSet(params: CreateRegionSetParams): Promise<CreateRegionSetResult> {
  const response = await http.post<ApiRegionSet, CreateRegionSetParams>("/region-sets/create", params);
  return mapApiRegionSet(response);
}

export function apiUpdateRegionSet(params: EditRegionSetParams): Promise<EditRegionSetResult> {
  return http.patch("/region-sets/edit", params);
}

export function apiRemoveRegionSet(params: RemoveRegionSetParams): Promise<void> {
  return http.delete(`/region-sets/delete?regionSetId=${params.regionSetId}`);
}

export async function apiCopyRegionSet(params: CopyRegionSetParams): Promise<CreateRegionSetResult> {
  const response = await http.post<ApiRegionSet, CopyRegionSetParams>("/region-sets/copy-region-set", params);
  return mapApiRegionSet(response);
}
