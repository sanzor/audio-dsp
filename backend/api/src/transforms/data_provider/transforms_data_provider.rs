use domain::{
    db::{
        db_transform::{DbTransform, TransformId},
        ticket::{
            create_ticket_params::CreateTransformDraftParams,
            db_resource::{DbResource, ResourceId},
            db_ticket::{DbTicket, TicketId},
        },
        WorkspaceId,
    },
    domain_user::UserId,
};

use crate::domain::data_error::DataError;

#[async_trait::async_trait]
pub trait TransformsDataProvider: Send + Sync {


    async fn list_transform_summaries(
        &self,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<DbTransform>, i64), DataError>;
    /// Catalog for one workspace — default transforms, caller-owned
    /// transforms, and transforms granted directly to the caller or workspace.
    async fn get_transforms_for_workspace_and_user(
        &self,
        user_id: UserId,
        workspace_id: WorkspaceId,
    ) -> Result<Vec<DbTransform>, DataError>;
    async fn get_transform(&self, id: TransformId) -> Result<DbTransform, DataError>;
    async fn get_transforms(&self, ids: &[TransformId]) -> Result<Vec<DbTransform>, DataError>;
    async fn get_transform_owner(&self, id: TransformId) -> Result<UserId, DataError>;
    /// All transform ids the user may read: defaults, owned, granted directly
    /// to them, or granted to a workspace they belong to. Backs
    /// `TransformAccessContext`, loaded once per request by
    /// `TransformAccessMiddleware` so handlers can check membership
    /// locally instead of a grants lookup per resource id.
    async fn list_accessible_transform_ids(
        &self,
        user_id: UserId,
    ) -> Result<Vec<TransformId>, DataError>;
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
