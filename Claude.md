# DAW Project Memory: Standards & Rules

## 🛠 Tech Stack
- **Backend:** Rust (Actix/Axum) - Handles audio graph, WASM transforms and metadata (regions,region sets, graphs)  and persistence.
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
- **Engine:** All audio transforms run in the browser via WASM.
- **Thread Safety:** UI/React Flow must never block the Audio Thread.
- **Audio Worklets:** Use AudioWorkletNode for all transforms. Do not process audio in the main React `useEffect` loops.
- **Destructive Edits:** When a user "Applies" a transform graph, the result is computed in the frontend and sent as a finished Blob/Buffer to the Rust backend for storage.

## 📦 WASM Management
- **Persistence:** Rust backend is strictly for saving/loading Region Sets and Transform Graphs.
- **State:** The React Flow graph is the "Source of Truth" for the current audio routing.