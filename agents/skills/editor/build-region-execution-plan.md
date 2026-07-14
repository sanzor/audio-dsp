# Skill: Editor / Build Region Execution Plan

Use this workflow when the editor must derive a deterministic transform chain from the graph attached to a region.

## Trigger

- graph logic changes
- execution order bugs appear
- editor AI needs to synthesize a runnable plan from graph edits

## Checklist

1. Identify the region graph source of truth.
2. Build the DAG view from the persisted or in-memory graph model.
3. Derive execution order and validate that it is acyclic and complete.
4. Resolve each graph node to a published transform identity and version.
5. Produce the runtime payload expected by the worklet pipeline.
6. Verify that derived order matches actual runtime order.

## Repo touchpoints

- editor graph state
- DAG derivation and selectors
- runtime plan compilation or mapping layer
- worklet handoff path

## Done when

- the region graph produces a deterministic execution plan
- transform ordering is explicit
- the runtime consumes the plan without ambiguity

## Minimum verification

- ordering is stable for the same graph
- invalid graph shapes fail clearly
- worklet execution order matches derived plan order
