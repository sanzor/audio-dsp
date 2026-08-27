use domain::{
    db::{
        db_transform::{DbTransform, TransformId},
        db_transform_draft::{DbTransformDraft, TransformDraftId},
    },
    domain_user::UserId,
};

use crate::domain::data_error::DataError;

#[async_trait::async_trait]
pub trait TransformDraftsDataProvider: Send + Sync {
    /// Creates a transform and its (bucket 2) draft row together, so the
    /// draft row is always present — save/publish never have to
    /// special-case "no row yet". `kind` is "primitive" | "composite",
    /// validated by the caller.
    async fn insert_transform_draft(
        &self,
        name: String,
        description: Option<String>,
        icon: Option<String>,
        kind: String,
        owner_user_id: UserId,
    ) -> Result<DbTransformDraft, DataError>;
    async fn get_transform_draft(&self, id: TransformDraftId) -> Result<DbTransformDraft, DataError>;
    async fn get_transform_drafts(&self, ids: &[TransformDraftId]) -> Result<Vec<DbTransformDraft>, DataError>;
    /// Cheap point-lookup for ownership checks — owner_user_id lives on the
    /// `transform` row, which a draft shares 1:1 with (see `TransformDraftId`'s
    /// doc comment), so this is its own query rather than a call into
    /// `TransformsDataProvider`.
    async fn get_transform_owner(&self, id: TransformDraftId) -> Result<UserId, DataError>;
    /// Only allowed when the transform has never been published — same
    /// guard, same cascade as `TransformsDataProvider::delete_transform`,
    /// which this deliberately does not call through to: this data provider
    /// owns the full lifecycle of the row a draft shares with its transform,
    /// with no runtime dependency on the published-transform slice.
    async fn delete_transform_draft(&self, id: TransformDraftId) -> Result<(), DataError>;
    /// Read-only lookup of already-published transforms, by their
    /// `TransformId` — a composite draft's wiring graph can reference them
    /// as leaves, and validating (or publishing) that graph needs their
    /// kind/ports. Silently omits any id that doesn't exist or was never
    /// published — see `TransformDraftsProviderService::fetch_leaf_defs`.
    async fn get_published_transforms(&self, ids: &[TransformId]) -> Result<Vec<DbTransform>, DataError>;

    /// Bucket 2 — "save", primitive only. Only ever touches `source_code` —
    /// `wasm_bytecode`/`wasm_source_code` are written separately, by the
    /// ticket worker as soon as a compile succeeds
    /// (`TransformsDataProvider::cache_compiled_binary_on_draft`), so a
    /// save never wipes out (or needs to attach) a compiled binary itself.
    async fn save_primitive_draft(
        &self,
        id: TransformDraftId,
        source_code: String,
    ) -> Result<DbTransformDraft, DataError>;
    /// Bucket 2 — "save", composite only. `graph_json` (the wiring graph)
    /// overwrites `transform_draft.metadata` wholesale — a composite draft
    /// has no other bucket-2 state to preserve alongside it.
    async fn save_composite_draft(
        &self,
        id: TransformDraftId,
        graph_json: String,
    ) -> Result<DbTransformDraft, DataError>;

    /// Atomically replaces the live transform's source/binary/metadata with
    /// what's currently saved (bucket 2), publishing it as bucket 3. One
    /// transaction so a transform's definition and its binary can never
    /// observably drift from each other.
    async fn publish_compiled_transform(
        &self,
        id: TransformDraftId,
        wasm_bytecode: Vec<u8>,
        source_code: String,
        name: String,
        description: Option<String>,
        metadata: String,
    ) -> Result<DbTransform, DataError>;

    /// Composite's publish — no binary, no source_code, just the derived
    /// `metadata` envelope (ports + graph). `source_code`/`wasm_bytecode`
    /// stay whatever they already were (`NULL`, for a composite, forever).
    async fn publish_composite_transform(
        &self,
        id: TransformDraftId,
        name: String,
        description: Option<String>,
        metadata: String,
    ) -> Result<DbTransform, DataError>;
}
