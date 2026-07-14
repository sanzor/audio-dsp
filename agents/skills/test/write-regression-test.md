# Skill: Test / Write Regression Test

Use this workflow after fixing a bug or hardening a risky creator, editor, or runtime path.

## Trigger

- a user-visible bug was fixed
- a creator/editor boundary broke before
- a runtime invariant needs a lasting guardrail

## Checklist

1. Capture the bug as a concrete behavior, not as implementation detail.
2. Place the test at the narrowest layer that can reproduce the issue reliably.
3. Prefer deterministic fixtures over broad integration setup.
4. Assert the failure mode and the expected corrected behavior.
5. Name the test after the user-visible bug or invariant.

## Preferred placement

- creator flow tests for compile/publish regressions
- editor tests for graph derivation, selectors, and UI behavior
- runtime tests for playback and execution-order regressions
- end-to-end scenario tests for creator-to-editor lifecycle regressions

## Done when

- the bug is reproducible through the test
- the expected fixed behavior is asserted
- the test lives at the smallest stable layer possible

## Minimum verification

- test fails before the fix when practical
- test passes with the fix
- related invariants in `agents/invariants.md` still read as true
