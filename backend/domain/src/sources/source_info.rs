use serde::{Deserialize, Serialize};

/// Metadata for a Creator-surface "signal source" -- a real audio file (name, extension,
/// duration) selectable in the shared "Try it" preview session, as an alternative to the
/// synthetic test tone. No regions/trim/gain/loop-point -- that complexity belongs to the
/// Editor's Track/Region domain, not here.
///
/// Not to be confused with the "source"/"sink" `nodeType` values inside Editor-side
/// `graph_state` JSONB (migration 0008_source_sink_nodes) -- an unrelated concept.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceInfo {
    pub name: String,
    pub extension: String,
    pub length: f32,
}
