# DAW Project Memory: Standards & Rules

## 🛠 Tech Stack
- **Backend:** Rust (Actix/Axum) - Handles transform compilation, tickets, metadata (regions, region sets, graphs), and persistence.
- **Frontend:** React + Shadcn UI + React Flow + Wavesurfer.js.


## ⚙️ Audio Engineering Rules
- **Non-Blocking UI:** Never run heavy calculations on the React main thread. Use Web Workers or offload to the Rust backend.
- **Wavesurfer Lifecycle:** Always use `useRef` for Wavesurfer instances. Ensure `.destroy()` is called in the `useEffect` cleanup.
- **React Flow Nodes:** Custom nodes must be memoized (`React.memo`) to prevent UI lag during playback.

## 🎨 Visual Identity
- Refer to `artifacts/stitch_output.html` for the source of truth on layout.
- Use the CSS variables defined in `src/styles/variables.css` (harvested from Stitch).
- Main Workspace Grid: `grid-template-columns: 240px 240px 100px 1fr;`

# Project Memory: UI-Centric DSP Rules

## 🚀 High-Performance UI
- **Engine:** Published transform chains run in the browser via WASM and AudioWorklet.
- **Thread Safety:** UI/React Flow must never block the Audio Thread.
- **Audio Worklets:** Use AudioWorkletNode for all transforms. Do not process audio in the main React `useEffect` loops.
- **Editor Runtime Boundary:** The editor may fetch and cache already-compiled transform binaries, but it must not trigger compilation.
- **Destructive Edits:** When a user "Applies" a transform graph, the result is computed in the frontend and sent as a finished Blob/Buffer to the Rust backend for storage.

## 📦 WASM Management
- **Compilation:** Transform source is compiled on the backend through the ticket flow.
- **Persistence:** Rust backend saves/loads Region Sets, Transform Graphs, artifacts, and related metadata.
- **State:** The React Flow graph is the "Source of Truth" for the current audio routing.
