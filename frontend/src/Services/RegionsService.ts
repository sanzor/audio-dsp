import type { TrackRegion } from "@/domain/Region/TrackRegion";
import type { TrackRegionSet } from "@/domain/RegionSet/TrackRegionSet";
import { http } from "@/Services/http";

interface ApiRegion {
  regionId?: number;
  region_id?: number;
  regionSetId?: number;
  region_set_id?: number;
  name: string;
  startTime?: number;
  start_time?: number;
  endTime?: number;
  end_time?: number;
  graph?: TrackRegion["graph"];
}

interface ApiRegionSet {
  id?: number;
  regionSetId?: number;
  region_set_id?: number;
  trackId?: number;
  track_id?: number;
  name: string;
  regions?: ApiRegion[];
}

const mapApiRegion = (region: ApiRegion): TrackRegion => ({
  regionId: region.regionId ?? region.region_id ?? 0,
  regionSetId: region.regionSetId ?? region.region_set_id ?? 0,
  name: region.name,
  start: region.startTime ?? region.start_time ?? 0,
  end: region.endTime ?? region.end_time ?? 0,
  graph: region.graph,
});

const mapApiRegionSet = (regionSet: ApiRegionSet): TrackRegionSet => ({
  id: regionSet.id ?? regionSet.regionSetId ?? regionSet.region_set_id ?? 0,
  trackId: regionSet.trackId ?? regionSet.track_id ?? 0,
  name: regionSet.name,
  regions: (regionSet.regions ?? []).map(mapApiRegion),
});

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
  regionSet: TrackRegionSet;
}

export interface CopyRegionParams {
  sourceRegionId: number;
  sourceRegionSetId: number;
  sourceTrackId: number;
  destinationRegionSetId: number;
  destinationTrackId: number;
  copyName: string;
}
export interface CopyRegionResult {
  region: TrackRegion;
}

export interface EditRegionParams {
  regionId: number;
  name?: string;
  startTime?: number;
  endTime?: number;
}
export interface EditRegionResult {
  region: TrackRegion;
}

export interface RemoveRegionParams {
  regionId: number;
}
export interface RemoveRegionResult {
  regionSet: TrackRegionSet;
}

export function apiGetRegion(_regionId: number): Promise<TrackRegion> {
  throw new Error("apiGetRegion is not implemented by the backend");
}

export function apiGetRegionsForRegionSet(_regionSetId: number): Promise<GetRegionsForRegionSetResult> {
  throw new Error("apiGetRegionsForRegionSet is not implemented by the backend");
}

export function apiAddRegion(params: CreateRegionParams): Promise<CreateRegionResult> {
  return http
    .post<ApiRegionSet, { regionSetId: number; startTime: number; endTime?: number; name: string }>(
      "/regions/add",
      {
        regionSetId: params.region_set_id,
        startTime: params.start_time,
        ...(params.end_time != null ? { endTime: params.end_time } : {}),
        name: params.name,
      },
    )
    .then((response) => ({ regionSet: mapApiRegionSet(response) }));
}

export function apiEditRegion(params: EditRegionParams): Promise<EditRegionResult> {
  return http
    .patch<{ region: ApiRegion }, { regionId: number; name?: string; startTime?: number; endTime?: number }>(
      "/regions/edit",
      {
      regionId: params.regionId,
      ...(params.name != null ? { name: params.name } : {}),
      ...(params.startTime != null ? { startTime: params.startTime } : {}),
      ...(params.endTime != null ? { endTime: params.endTime } : {}),
      },
    )
    .then((response) => ({ region: mapApiRegion(response.region) }));
}

export function apiRemoveRegion(params: RemoveRegionParams): Promise<RemoveRegionResult> {
  return http
    .delete<ApiRegionSet>(`/regions/remove?regionId=${params.regionId}`)
    .then((response) => ({ regionSet: mapApiRegionSet(response) }));
}

export function apiCopyRegion(params: CopyRegionParams): Promise<CopyRegionResult> {
  const query = new URLSearchParams({
    sourceRegionId: String(params.sourceRegionId),
    sourceRegionSetId: String(params.sourceRegionSetId),
    sourceTrackId: String(params.sourceTrackId),
    destinationRegionSetId: String(params.destinationRegionSetId),
    destinationTrackId: String(params.destinationTrackId),
    copyName: params.copyName,
  });

  return http
    .post<{ region: ApiRegion }, undefined>(`/regions/copy?${query.toString()}`, undefined)
    .then((response) => ({ region: mapApiRegion(response.region) }));
}
