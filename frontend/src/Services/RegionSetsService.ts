import type { TrackRegionSet } from "@/domain/RegionSet/TrackRegionSet";
import { http } from "@/Services/http";

// ─── Params & Results ────────────────────────────────────────────────────────

export interface GetRegionSetParams {
  region_set_id: number;
}
export interface GetRegionSetResult {
  region_set: TrackRegionSet;
}

export interface GetRegionSetsForTrackResult {
  trackId: number;
  sets: TrackRegionSet[];
}

export interface GetRegionSetsResult {
  track_region_sets_map: Map<number, TrackRegionSet[]>;
}

export interface CreateRegionSetParams {
  name: string | null;
  track_id: number;
}
export interface CreateRegionSetResult {
  region_set: TrackRegionSet;
}

export interface CopyRegionSetParams {
  sourceRegionSetId: number;
  destTrackId: number;
  copy_region_set_name: string;
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
  return http.get(`/region-sets/get-region-set?region_set_id=${regionSetId}`);
}

export function apiGetRegionSetsForTrack(trackId: number): Promise<GetRegionSetsForTrackResult> {
  return http.get(`/region-sets/get-all-for-track?track_id=${trackId}`);
}

export function apiGetAllRegionSets(): Promise<GetRegionSetsResult> {
  return http.get("/region-sets/get-region-sets");
}

export function apiCreateRegionSet(params: CreateRegionSetParams): Promise<CreateRegionSetResult> {
  return http.post("/region-sets/create", params);
}

export function apiUpdateRegionSet(params: EditRegionSetParams): Promise<EditRegionSetResult> {
  return http.patch("/region-sets/edit", params);
}

export function apiRemoveRegionSet(params: RemoveRegionSetParams): Promise<void> {
  return http.delete(`/region-sets/remove?region_set_id=${params.regionSetId}`);
}

export function apiCopyRegionSet(params: CopyRegionSetParams): Promise<CreateRegionSetResult> {
  return http.post("/region-sets/create", params);
}
