# Orchestrator Cheatsheet

The orchestrators glue React Query mutations to the normalized Zustand stores so the Track → RegionSet → Region hierarchy stays in sync.

## Track level
- `useCopyTrack` normalizes the copied tree via `normalizeTrackWithCascade` and pushes all region sets/regions into their stores before inserting the new track.
- `useDeleteTrack` optimistically removes the whole subtree through `cascadeDeleteTrack`, and Rollback invalidations rely on React Query.
- Use these hooks in the track context menu callbacks (e.g. `onCopy`, `onRemove`).

## Region set level
- `useCopyRegionSet` and `useDeleteRegionSet` now also update the parent track’s `region_sets_ids` so the UI immediately reflects additions/removals.
- `useDeleteRegionSet` snapshots both the region set and its regions so optimistic deletion can be rolled back if the API call fails.
- When wiring region-set context menus, the mutation handlers already emit normalized data to the stores; the component only needs to call `.mutate({ ...params })`.

## Region level
- `useCreateRegion`, `useCopyRegion`, `useEditRegion`, and `useDeleteRegion` (new) all flow the server responses through `normalizeRegionSetWithCascade`, which refreshes the affected region set + regions.
- `useDeleteRegion` uses `cascadeDeleteRegion` for optimistic updates and restores the snapshot on failure (including re-attaching the region ID to the parent set).
- Hook usage from the region context menu typically looks like:

```ts
const copyRegion = useCopyRegion();
const deleteRegion = useDeleteRegion();

copyRegion.mutate({ regionSetId, regionId, copyName });
	deleteRegion.mutate({ regionSetId, regionId, trackId });
```

## View-model selectors
- `src/Selectors/trackViewModels.ts` builds tree-shaped view models (`TrackMetaViewModel`, `TrackRegionSetViewModel`, `TrackRegionViewModel`, `GraphViewModel`) directly from the normalized Zustand stores.
- `useTrackMetaWithRegionsList` feeds components such as `AppSidebar`/`WaveformPlayer` that still expect the legacy `TrackMetaWithRegions` shape by flattening the view model’s nested regions.
- Controllers can grab a single entity without re-querying:

```ts
const track = useTrackViewModelById(selectedTrackId);
const regionSet = useRegionSetViewModel(selectedTrackId, regionSetId);
const region = useRegionViewModel(trackId, regionSetId, regionId);
```

This gives you “view” (API DTO), “normalized” (store), and “view model” (selector output) layers for every hierarchy level.

## Remaining work / TODOs
1. Graphs: `normalizeGraph` only stores flat graph metadata today. Once node/edge stores exist we should cascade into them and add graph-level mutations (copy/delete/update) similar to regions.
2. Track queries still assume a generic `['tracks']` key; if the backend exposes project-level filtering we can extend the invalidation keys again.
3. Region/graph services need validation (several endpoints are placeholders). The orchestrators expect the server to return an updated `TrackRegionSet` payload for create/copy/edit.
