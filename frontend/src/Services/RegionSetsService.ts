
import type { TrackRegionSet } from "@/Domain/RegionSet/TrackRegionSet";
import type { CopyRegionSetParams } from "@/Dtos/RegionSets/CoyRegionSetParams";
import type { CreateRegionSetParams } from "@/Dtos/RegionSets/CreateRegionSetParams";
import type { CreateRegionSetResult } from "@/Dtos/RegionSets/CreateRegionSetResult";
import type { EditRegionSetParams } from "@/Dtos/RegionSets/EditRegionSetParams";
import type { EditRegionSetResult } from "@/Dtos/RegionSets/EditRegionSetResult";
import type { GetRegionSetsForTrackResult } from "@/Dtos/RegionSets/GetRegionSetsForTrackResult";
import type { GetRegionSetsResult } from "@/Dtos/RegionSets/GetRegionSetsResult";
import type { RemoveRegionSetParams } from "@/Dtos/RegionSets/RemoveRegionSetParams";


const BASE_URL = import.meta.env.VITE_API_BASE_URL;

export async function apiGetRegionSet(regionSetId:string):Promise<TrackRegionSet>{
    const res = await fetch(`${BASE_URL}/region-sets/get-region-set?region_set_id=${regionSetId}`, {
    method: 'GET',
    credentials: 'include',
  });

  if (!res.ok) throw new Error('Failed to fetch session');

  const json = await res.json(); // ✅ await here
  console.log(json);
  return json.tracks;
}

export async function apiGetRegionSetsForTrack(trackId:string):Promise<GetRegionSetsForTrackResult>{
    const res = await fetch(`${BASE_URL}/region-sets/get-all-for-track?track_id=${trackId}`, {
    method: 'GET',
    credentials: 'include',
  });

  if (!res.ok) throw new Error('Failed to fetch session');

  const json = await res.json(); // ✅ await here
  console.log(json);
  return json.tracks;
}

export async function apiGetAllRegionSets():Promise<GetRegionSetsResult>{
    const res = await fetch(`${BASE_URL}/region-sets/get-region-sets`, {
    method: 'GET',
    credentials: 'include',
  });

  if (!res.ok) throw new Error('Failed to fetch session');

  const json = await res.json(); // ✅ await here
  console.log(json);
  return json.tracks;
}


export async function apiCreateRegionSet(params: CreateRegionSetParams): Promise<CreateRegionSetResult> {
  console.log("a");
  const res = await fetch(`${BASE_URL}/regions/create`, {
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

export async function apiUpdateRegionSet(params: EditRegionSetParams): Promise<EditRegionSetResult> {
  console.log("a");
  const res = await fetch(`${BASE_URL}/region-sets/edit`, {
    method: 'PATCH', // ✅ must be POST to send body
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

export async function apiRemoveRegionSet(params: RemoveRegionSetParams): Promise<void> {
  const res = await fetch(`${BASE_URL}/region-sets/remove?region_set_id=${params.regionSetId}`, {
    method: 'DELETE',
    credentials: 'include'
  });

  if (!res.ok) throw new Error('Refresh token failed');
}

export async function apiCopyRegionSet(params: CopyRegionSetParams): Promise<CreateRegionSetResult> {
  console.log("a");
  const res = await fetch(`${BASE_URL}/region-sets/create`, {
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