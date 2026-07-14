# Skill: Creator / Create Transform

Use this workflow when adding a new transform to the creator surface or revising an existing transform definition before compilation.

## Trigger

- a new transform needs to be authored
- an existing transform needs new parameters or behavior
- AI-assisted creator flow needs to scaffold source and metadata

## Checklist

1. Define the transform contract:
   - transform purpose
   - parameters
   - channel assumptions
   - sample-rate behavior
   - expected ordering constraints if used in a chain
2. Create or update source and metadata in the creator flow.
3. Ensure the output is suitable for the normal compile-ticket pipeline.
4. Update any catalog-facing metadata needed by the editor.
5. Add deterministic verification for the transform behavior.

## Repo touchpoints

- creator-facing frontend code
- transform metadata and DTO layers
- compile input models
- transform source or fixtures used by the compile pipeline

## Done when

- the transform source and metadata are valid for compilation
- the creator flow can submit the transform through the normal pipeline
- the published result would be meaningful to the editor surface

## Minimum verification

- compile inputs are structurally valid
- transform assumptions are explicit in code or tests
- at least one deterministic verification path exists
