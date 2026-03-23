# Technical Specification: DAW Node Integration

## 1. Node Definition: `AudioTrackNode`
**Visual Source:** `/artifacts/stitch/TrackComponent.tsx`
**Logic Source:** `src/nodes/AudioTrack.tsx`

### Handles (Ports)
- **Target (Input):** `audio-in` 
  - *Data:* `AudioNode` reference from previous node.
  - *Position:* Left-center (mapped to the "In" icon in Stitch).
- **Source (Output):** `audio-out`
  - *Data:* Processed `AudioNode` reference.
  - *Position:* Right-center (mapped to the "Out" icon in Stitch).
- **Control (Input):** `automation-vca`
  - *Data:* Float (0.0 - 1.0).
  - *Position:* Top-center.

### Wavesurfer.js Integration
- **Container:** Must be injected into the `div.waveform-slot` from the Stitch artifact.
- **Event Mapping:**
  - `wavesurfer.on('interaction')`: Must dispatch a `SEEK_ACTION` to the Rust backend.
  - `wavesurfer.on('finish')`: Trigger React Flow edge animation to signify end of stream.
- **Performance:** Use `sampleRate: 8000` (v7/v8 optimization) for background tracks to keep React Flow dragging smooth.

## 2. State Sync: React Flow <-> Rust
- **Node Data:** Every React Flow node has a `data.rustId` that matches a `uuid` in the Rust `audio-engine`.
- **Throttling:** Do not sync UI position changes to Rust more than once every 100ms.