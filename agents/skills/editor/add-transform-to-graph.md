# Skill: Editor / Add Transform To Graph

Use this workflow when a published transform needs to become usable inside the editor graph experience.

## Trigger

- a published transform should appear as an editor-usable node
- an existing graph node needs new ports or UI behavior
- a transform is available in the catalog but cannot yet be placed in a region graph

## Checklist

1. Define the editor node contract:
   - node identity
   - inputs and outputs
   - control ports
   - persisted graph shape
2. Add or update the graph node UI and interaction behavior.
3. Wire store normalization, selectors, and mutation flows.
4. Connect graph planning so the node contributes correctly to the region DAG.
5. Ensure runtime mapping can resolve the node to the published transform artifact.
6. Verify persistence, reload, and playback behavior.

## Repo touchpoints

- `frontend/src/editor`
- graph node definitions and editor interaction code
- state normalization and selectors
- graph persistence paths
- transform-catalog lookup and runtime mapping

## Done when

- the transform can be added to the editor graph
- the graph persists and reloads correctly
- the execution plan includes the transform in the expected order

## Minimum verification

- node renders correctly
- connecting and disconnecting edges behaves correctly
- graph persistence round-trips
- runtime mapping resolves to the expected published transform
