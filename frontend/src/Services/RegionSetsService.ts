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
  id?: number;
  regionSetId?: number;
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
  id: regionSet.id ?? regionSet.regionSetId ?? 0,
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
  track_region_sets_map: Map<number, TrackRegionSet[]>;
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

export function apiGetRegionSet(regionSetId: number): Promise<TrackRegionSet> {
  return http.get<ApiRegionSet>(`/region-sets/get?region_set_id=${regionSetId}`).then(mapApiRegionSet);
}

export function apiGetRegionSetsForTrack(trackId: number): Promise<GetRegionSetsForTrackResult> {
  return http
    .get<{ trackId: number; regionSets: ApiRegionSet[] }>(`/region-sets/get-all-for-track?trackId=${trackId}`)
    .then((response) => ({
      trackId: response.trackId,
      regionSets: response.regionSets.map(mapApiRegionSet),
    }));
}

export function apiGetAllRegionSets(): Promise<GetRegionSetsResult> {
  return http.get("/region-sets/get-all");
}

export function apiCreateRegionSet(params: CreateRegionSetParams): Promise<CreateRegionSetResult> {
  return http.post<ApiRegionSet>("/region-sets/create", params).then(mapApiRegionSet);
}

export function apiUpdateRegionSet(params: EditRegionSetParams): Promise<EditRegionSetResult> {
  return http.patch("/region-sets/edit", params);
}

export function apiRemoveRegionSet(params: RemoveRegionSetParams): Promise<void> {
  return http.delete(`/region-sets/delete?regionSetId=${params.regionSetId}`);
}

export function apiCopyRegionSet(params: CopyRegionSetParams): Promise<CreateRegionSetResult> {
  return http.post<ApiRegionSet>("/region-sets/copy-region-set", params).then(mapApiRegionSet);
}
