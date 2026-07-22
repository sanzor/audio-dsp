# Mission

This is a first-pass synthesis from `README.md` and `agents/architecture.md`, written so every agent in the roster starts from the same understanding of *why* this product exists, not just *what* it does. Treat it as a draft — correct it rather than working around it if it's wrong.

## What this is

A DAW (Digital Audio Workstation) SaaS platform built around one core idea: **separate the people who write audio processing code from the people who use it**, and give each group a surface built for their actual job. It is two-fold, not one product wearing two hats:

- **A marketplace for transform creators.** Developers/DSP authors write audio transform code and get a compile-ticket pipeline that turns it into a sandboxed, portable WASM artifact. Published transforms become **nodes** anyone can pull into a graph — authored by you, or by someone else entirely. The catalog is the tradeable unit; a creator's work has value independent of whether they ever touch the editor themselves.
- **A hands-on place to experiment with music directly.** The editor is a node-based, graphical DAG: artists apply transforms — destructively or non-destructively — over specific parts of a track (regions), previewing the result immediately as they iterate. This half of the product doesn't require writing any code, and doesn't require the marketplace to have anyone else's nodes in it — a single user's own published transforms are enough to build a working graph.

The platform's job is to make the handoff between these two — code to artifact to catalog to graph to previewed, running audio — reliable enough that neither half has to think about the other's internals.

## Why the split matters

Most DAWs either lock users into a fixed plugin format (opaque, hard to extend) or expose a full programming environment to everyone (powerful, but useless to someone who just wants to mix a track). This product's bet is that authoring and using are genuinely different jobs, deserving genuinely different surfaces, sharing one catalog and one runtime underneath.

That bet does **not** require two separate populations of people. The same person can be both a transform creator and an editor user — publish their own nodes, then immediately go build with them — and evidence from analogous tools (Max4Live, Reaktor, Faust patches) suggests that's actually the common case, not the exception. See `agents/market-research.md` for what's been learned about this so far, including the closest existing competitor making the same bet. What the split buys you either way: a creator's node is usable by *anyone* on the platform, not just its author, and an editor never has to leave their session to get a new capability.

See `agents/architecture.md` for how the split is actually implemented (Creator vs. Editor surfaces, the compile/save/publish pipeline, the worklet runtime).

## Who it's for

- Creators: developers/DSP authors who want to ship a working transform — for themselves, or for the catalog at large — without owning frontend, playback, or graph-execution concerns.
- Editors: artists and audio engineers who want to apply and preview a growing catalog of processing nodes over their tracks, destructively or non-destructively, without writing code.
- These are roles, not fixed people — see `agents/market-research.md` for open questions on who actually plays which role, and how often the same person plays both.

## Product surfaces referenced throughout the roster

- **Creator surface** — transform authoring, compiling, saving, publishing. Owned (dev-time) by `creator-agent`.
- **Editor surface** — track/region/graph composition, playback, worklet runtime. Owned (dev-time) by `editor-agent`.
- A planned **marketing/landing surface** does not exist in the codebase yet; see `agents/consultants/marketing-ui-expert.md` for when it's scoped.

## What this file is not

It's not a roadmap or a monetization plan — neither exists in the repo yet. Target-market analysis now lives in `agents/market-research.md`; this file states the product bet, that file tracks how well the bet is holding up against evidence.
