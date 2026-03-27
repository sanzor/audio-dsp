import type { TrackRegion } from "@/domain/Region/TrackRegion";
import { http } from "@/Services/http";

// ─── Params & Results ────────────────────────────────────────────────────────

export interface GetRegionsForRegionSetResult {
  regions: TrackRegion[];
}

export interface CreateRegionParams {
  start_time: number;
  end_time: number | null;
  name: string;
  region_set_id: number;
}
export interface CreateRegionResult {
  region: TrackRegion;
}

export interface CopyRegionParams {
  sourceRegionId: number;
  destinationRegionSetId: number;
  copyName: string;
}
export interface CopyRegionResult {
  region: TrackRegion;
}

export interface EditRegionParams {
  regionId: number;
  name: string;
}
export interface EditRegionResult {
  region: TrackRegion;
}

export interface RemoveRegionParams {
  regionId: number;
}
export interface RemoveRegionResult {
  result: string;
}

// ─── API ──────────────────────────────────────────────────────────────────────

export function apiGetRegion(regionId: number): Promise<TrackRegion> {
  return http.get(`/region-sets/get-region?region_set_id=${regionId}`);
}

export function apiGetRegionsForRegionSet(regionSetId: number): Promise<GetRegionsForRegionSetResult> {
  return http.get(`/regions/get-regions?region-set-id=${regionSetId}`);
}

export function apiAddRegion(params: CreateRegionParams): Promise<CreateRegionResult> {
  return http.post("/regions/add", params);
}

export function apiEditRegion(params: EditRegionParams): Promise<EditRegionResult> {
  return http.patch("/regions/edit", params);
}

export function apiRemoveRegion(params: RemoveRegionParams): Promise<RemoveRegionResult> {
  return http.delete(`/regions/remove?track_id=${params.regionId}`);
}

export function apiCopyRegion(params: CopyRegionParams): Promise<CopyRegionResult> {
  return http.patch("/regions/copy", params);
}
