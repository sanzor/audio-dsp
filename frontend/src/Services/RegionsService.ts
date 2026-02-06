import type { TrackRegion } from "@/Domain/Region/TrackRegion";
import type { CopyRegionParams } from "@/Dtos/Regions/CopyRegionParams";
import type { CopyRegionResult } from "@/Dtos/Regions/CopyRegionResult";
import type { CreateRegionParams } from "@/Dtos/Regions/CreateRegionParams";
import type { CreateRegionResult } from "@/Dtos/Regions/CreateRegionResult";
import type { EditRegionParams } from "@/Dtos/Regions/EditRegionParams";
import type { EditRegionResult } from "@/Dtos/Regions/EditRegionResult";
import type { GetRegionsForRegionSetResult } from "@/Dtos/Regions/GetRegionsForSetResult";
import type { RemoveRegionParams } from "@/Dtos/Regions/RemoveRegionParams";
import type { RemoveRegionResult } from "@/Dtos/Regions/RemoveRegionResult";

const BASE_URL = import.meta.env.VITE_API_BASE_URL;

export async function apiGetRegion(regionId:string):Promise<TrackRegion>{
    const res = await fetch(`${BASE_URL}/region-sets/get-region?region_set_id=${regionId}`, {
    method: 'GET',
    credentials: 'include',
  });

  if (!res.ok) throw new Error('Failed to fetch session');

  const json = await res.json(); // ✅ await here
  console.log(json);
  return json.tracks;
}


export async function apiGetRegionsForRegionSet(regionSetId:string):Promise<GetRegionsForRegionSetResult>{
    const res = await fetch(`${BASE_URL}/regions/get-regions?region-set-id=${regionSetId}`, {
    method: 'GET',
    credentials: 'include',
  });

  if (!res.ok) throw new Error('Failed to fetch region');

  const json = await res.json(); // ✅ await here
  console.log(json);
  return json.tracks;
}


export async function apiAddRegion(params: CreateRegionParams): Promise<CreateRegionResult> {
  console.log("a");
  const res = await fetch(`${BASE_URL}/regions/add`, {
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

export async function apiEditRegion(params: EditRegionParams): Promise<EditRegionResult> {
  console.log("a");
  const res = await fetch(`${BASE_URL}/regions/edit`, {
    method: 'PATCH', // ✅ must be POST to send body
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(params),
  });
  console.log(res.status);
  if (!res.ok) {
    throw new Error(`Failed to update region: ${res.statusText}`);
  }
  return await res.json();
}

export async function apiRemoveRegion(params: RemoveRegionParams): Promise<RemoveRegionResult> {
  const res = await fetch(`${BASE_URL}/regions/remove?track_id=${params.regionId}`, {
    method: 'DELETE',
    credentials: 'include'
  });

  if (!res.ok) throw new Error('Failed to remove region');
  return await res.json();
}

export async function apiCopyRegion(params:CopyRegionParams):Promise<CopyRegionResult>{
  const res = await fetch(`${BASE_URL}/regions/copy`, {
    method: 'PATCH', // ✅ must be POST to send body
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(params),
  });
  console.log(res.status);
  if (!res.ok) {
    throw new Error(`Failed to update region: ${res.statusText}`);
  }
  return await res.json();
}