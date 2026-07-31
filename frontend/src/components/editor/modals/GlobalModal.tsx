import { TrackModals } from "./track/track-modals";
import { RegionSetModals } from "./region-set/region-set-modals";
import { RegionModals } from "./region/region-modals";
import { GraphModals } from "./graph/graph-modals";
import { TransformModals } from "./transform/transform-modals";

export function GlobalModal() {
  return (
    <>
      <TrackModals />
      <RegionSetModals />
      <RegionModals />
      <GraphModals />
      <TransformModals />
    </>
  );
}
