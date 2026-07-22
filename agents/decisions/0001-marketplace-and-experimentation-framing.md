# 0001: Two-fold marketplace + experimentation framing, not a strict two-population split

- **Status:** Accepted
- **Date:** 2026-07-20

## Context

The product's original framing (`agents/mission.md`, pre-2026-07-20) described two distinct populations sharing one platform: transform creators who write code and never touch the editor, and editor users who compose graphs and never write code.

A `business-analyst` research pass (`agents/market-research.md`, Round 1) found this was the single biggest unvalidated risk in the product: the closest real-world analogues to "someone writes DSP, someone else uses it" (Max4Live, Reaktor, Faust) show that in practice it's almost always the *same person* wearing both hats, building for themselves or a small hobbyist circle — not two distinct populations transacting through a shared catalog at meaningful scale. No evidence was found of the strict split happening anywhere.

Separately, the user clarified the actual intended shape of the editor experience: a node-based graphical DAG applied over specific parts of a track (regions), destructively or non-destructively, with immediate preview — not just "graph composition" in the abstract.

## Decision

Reframe the product as two-fold, not two-population:

1. **A marketplace for transform creators** — published transforms become nodes usable by anyone, authored by the user themselves or by someone else. The catalog is the tradeable unit; a creator's work has value independent of whether they personally use the editor.
2. **A hands-on experimentation tool** — the editor applies transforms (destructively or non-destructively) over track regions with live preview. This half works standalone: a single user's own published nodes are sufficient, no marketplace participation required.

The same person playing both roles is now treated as the expected, common case — not a failure of the premise.

## Consequences

- `agents/mission.md` rewritten: "What this is," "Why the split matters," and "Who it's for" sections all updated to state the two-fold framing and explicitly note that one person may play both roles.
- `agents/architecture.md`'s "Surface 2: Editor" section updated to name the destructive/non-destructive distinction and marketplace-sourced nodes explicitly, where it was previously only implicit.
- `agents/market-research.md` gained a "Round 1.5" note capturing the reframe, and a "Round 2" research pass re-run against the new framing (found: strong evidence the marketplace half works independent of same-person usage — a single Reaktor ensemble was downloaded 11,800+ times in one year; and a possible genuine differentiator — no competitor checked, including Bitwig's Grid, Ableton's Racks, or Audiotool, was found to support region-scoped graph application to an arbitrary sub-span inside a clip).
- Open follow-up, not yet decided: whether to prioritize validating the region-scoping differentiator with real users, or to test-drive Audiotool NEXUS directly to confirm whether its patching model is destructive or non-destructive (couldn't be confirmed from public docs). See `agents/market-research.md` Round 2, section 5.
