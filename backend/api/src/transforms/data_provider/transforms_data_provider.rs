use domain::{
    db::{
        WorkspaceId,
        db_transform::{DbTransform, TransformId},
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
    /// All transform ids the user may read: owned, granted directly to
    /// them, or granted to a workspace they belong to. Backs
    /// `TransformAccessContext`, loaded once per request by
    /// `TransformAccessMiddleware` so handlers can check membership
    /// locally instead of a grants lookup per resource id.
    async fn list_accessible_transform_ids(&self, user_id: UserId) -> Result<Vec<TransformId>, DataError>;
    /// Only allowed when the transform has never been published (its
    /// `wasm_bytecode` is still empty) — see
    /// `agents/decisions/0002-transform-draft-lifecycle-decisions.md`.
    /// Cascades to `transform_draft`/`transform_ticket`/`transform_resource`
    /// via existing FK `ON DELETE CASCADE`. Also the backing implementation
    /// for `TransformDraftsProviderService::delete_transform_draft` — a
    /// draft and its transform share the same underlying row id, see
    /// `TransformDraftId`'s doc comment.
    async fn delete_transform(&self, id: TransformId) -> Result<(), DataError>;
}
