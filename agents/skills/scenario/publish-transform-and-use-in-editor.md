# Skill: Scenario / Publish Transform And Use In Editor

Use this workflow for the highest-value cross-surface path: creator produces a transform, editor consumes it, and runtime executes it.

## Trigger

- end-to-end validation of the transform lifecycle is needed
- a new transform must be proven usable in the editor
- a regression may live at the creator/editor boundary

## Checklist

1. Create or update the transform in the creator flow.
2. Submit compile and wait for success.
3. Publish the transform and confirm it appears in the catalog.
4. Open the editor flow and fetch the published binary.
5. Add the transform to a region graph.
6. Build the region execution plan.
7. Run or preview the chain through the frontend worklet runtime.
8. Save and reload if persistence is part of the scenario.

## Repo touchpoints

- creator frontend and APIs
- compile-ticket workflow
- transform catalog
- editor graph flows
- runtime/worklet execution path

## Done when

- the published transform is discoverable in the editor
- the transform can be attached to a region graph
- the frontend runtime executes it in the expected chain

## Minimum verification

- compile succeeded
- catalog exposure succeeded
- editor fetch/cache succeeded
- graph execution succeeded
