# Skill: Editor / Fetch And Cache Transform Binary

Use this workflow when the editor needs to retrieve a published transform artifact and make it available to the frontend runtime.

## Trigger

- a published transform must be loaded for execution
- cache invalidation or versioning behavior changes
- editor AI needs binaries available before applying a generated graph

## Checklist

1. Identify the published transform id and version.
2. Fetch the binary through the editor-allowed artifact path.
3. Cache the artifact with explicit version semantics.
4. Define invalidation or refresh behavior for updated transform versions.
5. Verify the runtime can consume the cached artifact.

## Repo touchpoints

- editor frontend data-fetching code
- artifact retrieval APIs
- frontend cache/storage layer
- runtime/worklet loading path

## Done when

- the editor can fetch a published transform binary
- cached artifacts are version-aware
- runtime loading uses the cached artifact or refreshes correctly

## Minimum verification

- first-load fetch works
- repeat load uses cache when expected
- version change causes the expected refresh path
