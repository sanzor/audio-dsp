import type { TrackRegion } from "@/domain/Region/TrackRegion";
import type { TrackRegionSet } from "@/domain/RegionSet/TrackRegionSet";
import { http, projectApiPath } from "@/Services/http";

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

export async function apiAddRegion(params: CreateRegionParams): Promise<CreateRegionResult> {
  const response = await http.post<ApiRegionSet, { regionSetId: number; startTime: number; endTime?: number; name: string }>(
    projectApiPath("/regions/add"),
    {
      regionSetId: params.region_set_id,
      startTime: params.start_time,
      ...(params.end_time != null ? { endTime: params.end_time } : {}),
      name: params.name,
    },
  );
  return { regionSet: mapApiRegionSet(response) };
}

export async function apiEditRegion(params: EditRegionParams): Promise<EditRegionResult> {
  const response = await http.patch<{ region: ApiRegion }, { regionId: number; name?: string; startTime?: number; endTime?: number }>(
    projectApiPath("/regions/edit"),
    {
      regionId: params.regionId,
      ...(params.name != null ? { name: params.name } : {}),
      ...(params.startTime != null ? { startTime: params.startTime } : {}),
      ...(params.endTime != null ? { endTime: params.endTime } : {}),
    },
  );
  return { region: mapApiRegion(response.region) };
}

export async function apiRemoveRegion(params: RemoveRegionParams): Promise<RemoveRegionResult> {
  const response = await http.delete<ApiRegionSet>(projectApiPath(`/regions/remove?regionId=${params.regionId}`));
  return { regionSet: mapApiRegionSet(response) };
}

export async function apiCopyRegion(params: CopyRegionParams): Promise<CopyRegionResult> {
  const query = new URLSearchParams({
    sourceRegionId: String(params.sourceRegionId),
    sourceRegionSetId: String(params.sourceRegionSetId),
    sourceTrackId: String(params.sourceTrackId),
    destinationRegionSetId: String(params.destinationRegionSetId),
    destinationTrackId: String(params.destinationTrackId),
    copyName: params.copyName,
  });

  const response = await http.post<{ region: ApiRegion }, undefined>(projectApiPath(`/regions/copy?${query.toString()}`), undefined);
  return { region: mapApiRegion(response.region) };
}
