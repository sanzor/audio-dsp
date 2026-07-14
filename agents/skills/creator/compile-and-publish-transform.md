# Skill: Creator / Compile And Publish Transform

Use this workflow when a transform should move from authored source to an editor-usable published artifact.

## Trigger

- creator source is ready to compile
- a transform version needs to be rebuilt or republished
- an AI-generated transform needs to go through the standard release path

## Checklist

1. Validate the transform input and metadata.
2. Submit the backend compile ticket.
3. Poll or inspect compile status and errors.
4. Fix source or metadata if the compile fails.
5. Publish the compiled transform and version metadata.
6. Verify it becomes available in the editor-facing transform catalog.

## Repo touchpoints

- creator API flows
- backend compile-ticket and worker flows
- transform metadata persistence
- editor transform catalog/discovery path

## Done when

- the transform compiles successfully
- the artifact is stored and versioned
- the editor can discover the published transform

## Minimum verification

- compile ticket reaches success state
- published transform metadata is queryable
- editor-facing catalog includes the new version
