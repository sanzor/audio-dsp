use domain::{
    db::{
        WorkspaceId,
        db_transform::{DbTransform, TransformId},
        db_transform_draft::{DbTransformDraft, TransformDraftId},
        ticket::{
            create_ticket_params::CreateTicketParams,
            db_resource::{DbResource, ResourceId},
            db_ticket::{DbTicket, TicketId},
            update_ticket_params::UpdateTicketParams,
        },
    },
    domain_user::UserId,
};

use crate::domain::data_error::DataError;

#[async_trait::async_trait]
pub trait TransformsDataProvider: Send + Sync {
    // ── Bucket 1 — compile tickets / resources ──────────────────────────

    async fn create_transform_ticket(&self, ticket: CreateTicketParams) -> Result<DbTicket, DataError>;
    async fn get_ticket(&self, ticket_id: TicketId) -> Result<DbTicket, DataError>;
    /// Point-lookup used for authorization — resolves which transform a
    /// ticket belongs to without fetching the full ticket.
    async fn get_ticket_transform_id(&self, ticket_id: TicketId) -> Result<TransformId, DataError>;
    /// Point-lookup used for authorization — resolves which transform a
    /// resource belongs to without fetching the full resource.
    async fn get_resource_transform_id(&self, resource_id: ResourceId) -> Result<TransformId, DataError>;
    async fn update_ticket(&self, ticket: UpdateTicketParams) -> Result<DbTicket, DataError>;

    /// Stores the full artifact a successful compile ticket produced —
    /// bucket 1. Immutable history; never touches bucket 2 (save) or
    /// bucket 3 (published) state. `metadata` is the raw JSON the compiled
    /// module's `metadata()` export produced (name/description/ports/params),
    /// already validated by `metadata_introspector`.
    async fn create_resource(
        &self,
        ticket_id: TicketId,
        wasm_bytecode: Vec<u8>,
        name: String,
        description: Option<String>,
        metadata: String,
    ) -> Result<DbResource, DataError>;
    async fn get_compiled_transform(&self, resource_id: ResourceId) -> Result<DbResource, DataError>;

    // ── Published transforms (bucket 3) ─────────────────────────────────

    async fn list_transform_summaries(&self, offset: i64, limit: i64) -> Result<(Vec<DbTransform>, i64), DataError>;
    /// Catalog for one workspace — owned by the caller, granted directly to
    /// the caller, or granted to the workspace.
    async fn get_transforms_for_workspace_and_user(&self, user_id: UserId, workspace_id: WorkspaceId) -> Result<Vec<DbTransform>, DataError>;
    async fn get_transform(&self, id: TransformId) -> Result<DbTransform, DataError>;
    async fn get_transforms(&self, ids: &[TransformId]) -> Result<Vec<DbTransform>, DataError>;
    async fn get_transform_owner(&self, id: TransformId) -> Result<UserId, DataError>;
    /// Only allowed when the transform has never been published (its
    /// `wasm_bytecode` is still empty) — see
    /// `agents/decisions/0002-transform-draft-lifecycle-decisions.md`.
    /// Cascades to `transform_draft`/`transform_ticket`/`transform_resource`
    /// via existing FK `ON DELETE CASCADE`.
    async fn delete_transform(&self, id: TransformId) -> Result<(), DataError>;

    // ── Drafts (bucket 2) ────────────────────────────────────────────────

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
    /// Only allowed when the transform has never been published — same
    /// guard as `delete_transform` (today `TransformDraftId` and
    /// `TransformId` share the same underlying row, just kept as distinct
    /// Rust types so a draft id and a published id can't be swapped by
    /// accident at a call site).
    async fn delete_transform_draft(&self, id: TransformDraftId) -> Result<(), DataError>;

    /// Bucket 2 — "save". Always overwrites source_code. If `resource_id` is
    /// given, also copies that resource's (bucket 1) binary/metadata into the
    /// draft; the resource must belong to this transform and correspond to
    /// the exact source being saved. If omitted, any previously saved
    /// binary/metadata is left untouched — a source-only save never wipes
    /// out the last good build.
    async fn save_transform_draft(
        &self,
        id: TransformDraftId,
        source_code: String,
        resource_id: Option<ResourceId>,
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
}
