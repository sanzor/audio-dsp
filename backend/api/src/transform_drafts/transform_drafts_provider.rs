use domain::{
    db::{
        db_transform::DbTransform,
        db_transform_draft::{DbTransformDraft, TransformDraftId},
    },
    domain_user::UserId,
};

use crate::{
    domain::service_error::ServiceError,
    ticket_worker::processor::transform_metadata::PortMetadataJson,
    transform_drafts::dto::requests::SaveDraftParams,
};

#[async_trait::async_trait]
pub trait TransformDraftsProvider: Send + Sync {
    async fn get_transform_draft(
        &self,
        id: TransformDraftId,
    ) -> Result<DbTransformDraft, ServiceError>;
    async fn get_transform_drafts(
        &self,
        ids: &[TransformDraftId],
    ) -> Result<Vec<DbTransformDraft>, ServiceError>;
    /// Cheap point-lookup for ownership checks.
    async fn get_transform_draft_owner(&self, id: TransformDraftId)
        -> Result<UserId, ServiceError>;
    /// `kind` is "primitive" | "composite" — validated by the controller.
    async fn create_transform_draft(
        &self,
        name: String,
        description: Option<String>,
        icon: Option<String>,
        kind: String,
        owner_user_id: UserId,
    ) -> Result<DbTransformDraft, ServiceError>;
    /// Bucket 2 — save, primitive only. Errors if `id` is a composite draft.
    /// An optional frontend-held compiled WASM payload is saved atomically
    /// with source after server-side introspection.
    async fn save_draft(
        &self,
        id: TransformDraftId,
        params: SaveDraftParams,
    ) -> Result<DbTransformDraft, ServiceError>;
    async fn check_source(&self, source_code: String) -> Result<(), ServiceError>;
    async fn validate_graph_draft(
        &self,
        id: TransformDraftId,
        graph_json: String,
    ) -> Result<Vec<PortMetadataJson>, ServiceError>;

    async fn publish(&self, id: TransformDraftId) -> Result<DbTransform, ServiceError>;

    async fn delete_transform_draft(&self, id: TransformDraftId) -> Result<(), ServiceError>;
}
