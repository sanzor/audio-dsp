import { useUIStore } from "@/Stores/UIStore"
import { useRegionStore } from "@/Stores/RegionStore"
import { useRegionSetStore } from "@/Stores/RegionSetStore"

export function useCanDropTransform(): { canDropTransform: boolean; regionId: number | null } {
  const activeSelection = useUIStore((s) => s.activeSelection)
  const region = useRegionStore((s) =>
    activeSelection.regionId != null ? s.regions.get(activeSelection.regionId) : undefined
  )
  const regionSet = useRegionSetStore((s) =>
    activeSelection.regionSetId != null ? s.regionSets.get(activeSelection.regionSetId) : undefined
  )
  const canDropTransform =
    region != null &&
    regionSet != null &&
    region.regionSetId === regionSet.id &&
    regionSet.region_ids.includes(region.regionId)

  return { canDropTransform, regionId: activeSelection.regionId ?? null }
}
