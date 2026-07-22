# Architecture

## Product model

This platform serves two distinct user groups:

- transform creators
- audio artists and editors

Those groups share infrastructure and stored assets, but they do different work and need different UI flows.

## Surface 1: Creator

The creator surface is where a developer or DSP author writes transform code and submits it for compilation.

Core responsibilities:

- author transform source code
- define transform metadata and parameters
- submit compile jobs
- inspect compile status and build output
- publish or version transforms so they can be used in editor graphs

This is not the DAW graph-editing experience. It is a transform authoring and packaging workflow.

Planned extension:

- a chat interface may help the user generate or edit transform source files directly in the UI
- the AI may scaffold metadata, parameters, and source structure
- generated source still goes through the same compile-ticket and publish flow as manually written source

## Surface 2: Editor

The editor surface is where an artist or audio engineer builds audio work by combining existing transforms into a node-based graphical DAG, applied over specific parts of a track (regions) — destructively or non-destructively — with immediate preview as they iterate. See `agents/mission.md` for why this is framed as hands-on experimentation, not just "graph composition."

Core responsibilities:

- load tracks and regions
- drag and drop nodes — sourced from the published catalog, whether authored by this user or by someone else (see `agents/mission.md`'s marketplace framing)
- link transforms into graphs
- fetch and cache published transform binaries for runtime use
- attach graphs to regions or other editing scopes
- preview, iterate, and apply processing — the distinction between a destructive apply (renders/replaces audio) and non-destructive (graph stays live, re-editable) must stay explicit per feature; see the "Decision rule for future changes" below

This is the DAW-facing experience. It should optimize for graph clarity, playback responsiveness, and editing speed.

Planned extension:

- a chat interface may help the user build a graph over a selected audio region
- the AI may inspect audio context, region context, and the published transform catalog
- the AI may propose or apply graph edits equivalent to drag-and-drop editor actions
- the AI may save the resulting graph through the normal editor persistence path

## Recommended frontend split

Yes, the frontend should be split conceptually into these two parts:

- `creator` frontend domain
- `editor` frontend domain

Recommended first step:

- keep one repository
- keep one deployable frontend unless release pressure forces separation
- split code by route, feature folder, state ownership, and UI shell

That gives you separation of concerns without paying the cost of two independent frontend apps too early.

A practical shape would be:

- `frontend/src/creator/*`
- `frontend/src/editor/*`
- `frontend/src/shared/*`

Planned AI-oriented subdomains can sit inside those surfaces, for example:

- `frontend/src/creator/chat/*`
- `frontend/src/editor/chat/*`

Use `shared` only for genuinely shared concerns such as auth, design primitives, API client wiring, and common entity types. Do not let graph-editor state or creator compile state leak into the same feature modules.

## Backend shape

The backend has at least two major responsibilities:

1. platform/data APIs
2. transform compilation and processing workflows

### Platform/data APIs

These cover:

- auth
- projects
- workspaces
- tracks
- regions
- graphs
- transform metadata
- persisted artifacts and references

This is primarily represented today by:

- `backend/api`
- `backend/domain`
- `database/`

### Compilation workflow

Transform creation is not an immediate inline operation. The backend compiles transforms through a poll-based ticket system.

The expected flow is:

1. creator submits transform source and metadata
2. backend creates a compile ticket
3. worker or compiler process picks up the ticket
4. creator frontend polls ticket status
5. backend stores build result, errors, and produced artifact metadata as a compile resource (not yet live)
6. the creator explicitly saves and then publishes before a transform becomes available to the editor surface — a successful compile ticket alone does not do this

This compile pipeline should be treated as a product subsystem, not as a side detail of the editor.

WASM compilation is a backend concern. The creator frontend submits source and polls status, but it does not compile transforms locally.

That remains true even when source is AI-generated inside the creator UI. AI-assisted authoring changes how source is produced, not how compilation works.

After a transform is successfully compiled and published, it becomes part of the editor-side transform catalog.
The editor may fetch those published binaries and cache them locally for runtime use, but it does not submit compile jobs or trigger backend compilation.

Creator source compiles against `backend/transform-sdk` — see `agents/transforms.md` for the full ABI, compilation pipeline, and metadata-validation details.

A transform's in-progress state is split into three independently-writable buckets — compile (check), save, and publish — with an explicit action required to move between them. A successful compile ticket never auto-publishes; see `agents/transforms.md` for the full model.

## Editor graph execution planning

The editor should do as much graph-assembly work locally as is practical, provided it does not hurt interaction or playback responsiveness.

That includes frontend-owned tasks such as:

- building the region graph from user edits
- deriving ordered transform lists from a DAG
- validating graph shape before save or preview
- preparing the payload that will be persisted or handed off to runtime layers

This is different from compiling transform code. The editor may assemble and analyze graphs in the frontend, while transform compilation still happens on the backend.

The same rule applies to an agentic editor flow. The AI may inspect the available transform store and synthesize a graph plan for a region, but it must stay within editor responsibilities:

- use published transforms only
- create explicit graph edits
- persist through the same graph save path as manual UI actions

## Runtime and audio layers

Audio runtime concerns currently span:

- `backend/audiolib`
- `backend/player`
- frontend playback and graph interaction code

The execution model is hybrid and should be treated as the current architecture:

- transform code compilation happens on the backend
- compiled transforms are stored and published by the platform
- the editor may fetch and cache published transform binaries on the frontend
- editor-side DAG construction and graph-derived transform planning happen on the frontend
- once a transform is available to the editor, the runtime chain executes on the frontend through an audio worklet pipeline

What still needs to stay explicit per feature is not whether frontend execution exists, but the exact boundary of responsibility around preview, render, persistence, and playback semantics.

Every feature that touches transforms or playback must state:

- whether it belongs to creator compile flow or editor graph flow
- where DSP executes
- where graph truth lives during editing
- what gets persisted
- whether output is preview-only or a stored rendered artifact

## Current codebase surfaces

Today the repository is organized roughly like this:

- `frontend/`: React application, graph UI, state orchestration, playback-facing UX
- `backend/api`: HTTP/API layer, auth, worker integration, app services
- `backend/audiolib` and `backend/player`: audio utilities, playback primitives, tests
- `database/`: migrations, stored procedures, seed data, transform assets and metadata support

The root `Makefile` remains the operational entrypoint for local stack management, migrations, linting, and tests.

## Architectural boundaries

### Creator boundary

The creator surface owns:

- code authoring UX
- AI-assisted source generation and editing UX
- compile submission UX
- build status UX
- transform packaging metadata
- client-side "try it" preview execution of a just-compiled (not yet saved/published) binary — a narrow, preview-only runtime dependency on the Editor's worklet machinery (`graph-worklet.js` / `WorkletMessageSender`), reused directly so preview can't diverge from post-publish playback; see `agents/decisions/0003-transform-preview-flow.md` and the worklet dependency noted in `agents/ownership.md`'s Shared zones

It should not own DAW graph editing concerns, and this preview dependency does not extend to reusing the Editor's stateful worklet controller/hooks (`WorkletController`, `useWorkletSetup`) or its global playback state.

### Editor boundary

The editor surface owns:

- graph editing
- AI-assisted graph authoring UX
- region and track processing flows
- playback and waveform interaction
- transform selection and graph composition
- fetching and caching published transform binaries
- frontend execution of published transform chains through the worklet runtime

It should consume published transforms, not define how transforms are authored internally or trigger compilation. The worklet runtime module and message protocol are also relied on by the Creator surface's preview-only flow (read-only reuse, not a fork) — a change to the worklet message protocol or its API must be checked against that dependency too, not just editor playback.

### Shared boundary

Shared platform capabilities include:

- auth and identity
- project/workspace ownership
- storage and persistence
- transform catalog/discovery
- ticket status and artifact retrieval APIs
- chat/session plumbing for AI-assisted creator and editor experiences

## Decision rule for future changes

When implementing a feature, write down:

- is this for a creator or an editor
- is it manual UX, AI-assisted UX, or shared infrastructure
- does it belong to authoring, compilation, catalog, graph editing, playback, or persistence
- where the DSP executes
- who owns the source of truth at that step
- what tests cover the risk

If a change alters those boundaries, update this file in the same change.
