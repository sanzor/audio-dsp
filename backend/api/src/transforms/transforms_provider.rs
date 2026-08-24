use domain::{db::{
    WorkspaceId, db_transform::{DbTransform, TransformId},
     db_transform_draft::{DbTransformDraft, TransformDraftId}, ticket::db_resource::ResourceId,
}, domain_user::UserId};

use crate::domain::service_error::ServiceError;
use crate::ticket_worker::processor::transform_metadata::PortMetadataJson;

#[async_trait::async_trait]
pub trait TransformsProvider: Send + Sync {
    async fn list_transform_summaries(&self, offset: i64, limit: i64) -> Result<(Vec<DbTransform>, i64), ServiceError>;
    /// Catalog for one workspace — see `TransformsDataProvider::list_transforms_for_workspace_and_user`.
    async fn get_transforms_for_workspace_and_user(&self, user_id: UserId, workspace_id: WorkspaceId) -> Result<Vec<DbTransform>, ServiceError>;
    async fn get_transform(&self, id: TransformId) -> Result<DbTransform, ServiceError>;
    async fn get_transform_draft(&self, id: TransformDraftId) -> Result<DbTransformDraft, ServiceError>;
    async fn get_transforms(&self, ids: &[TransformId]) -> Result<Vec<DbTransform>, ServiceError>;
    async fn get_transform_drafts(&self, ids: &[TransformDraftId]) -> Result<Vec<DbTransformDraft>, ServiceError>;
    /// Cheap point-lookup for ownership checks.
    async fn get_transform_owner(&self, id: TransformId) -> Result<UserId, ServiceError>;
    /// `kind` is "primitive" | "composite" — validated by the controller.
    async fn create_transform_draft(&self,
        name: String,
        description: Option<String>, 
        icon: Option<String>,
        kind: String,
        owner_user_id: UserId) -> Result<DbTransformDraft, ServiceError>;
    /// Bucket 2 — save. Always overwrites source_code; if `resource_id` is
    /// given, also attaches that compile resource's binary/metadata.
    async fn save_transform_draft(&self,
         id: TransformDraftId, 
         source_code: String,
         resource_id: Option<ResourceId>) -> Result<DbTransformDraft, ServiceError>;
    
    /// "Bucket 2 publishable?" — source and binary present and in sync, i.e.
    /// whether `publish_transform` would succeed right now. A readiness
    /// check on already-saved state, not a correctness check on code — see
    /// `check_source` for that. Only meaningful for a primitive draft.
    async fn is_draft_publishable(&self, id: TransformDraftId) -> Result<bool, ServiceError>;
    /// Fast `cargo check` (type/borrow-check, no codegen) of `source_code`
    /// against the transform-sdk contract — not tied to what's saved, so the
    /// Creator can check live in-progress edits. `Ok(())` means it compiles
    /// cleanly; `Err` carries compiler diagnostics. Meant to be called
    /// synchronously for quick editor feedback; producing a real wasm
    /// artifact still goes through the ticket pipeline (`create_transform_ticket`).
    async fn check_source(&self, source_code: String) -> Result<(), ServiceError>;
    /// Validates a composite draft's wiring graph JSON (not necessarily what
    /// was last saved — the caller can pass live in-progress edits) against
    /// every already-published transform it references, and returns the
    /// composite's derived ports on success. See `validator::Validator` —
    /// this never recurses into a referenced composite's own graph, it only
    /// trusts that composite's already-published, already-validated ports.
    async fn validate_graph_draft(&self, id: TransformDraftId, metadata_json: String) -> Result<Vec<PortMetadataJson>, ServiceError>;
    /// Bucket 3 — publish. Bundles whatever's currently saved (bucket 2) into
    /// the live artifact. Fails if nothing has been saved with a binary yet.
    async fn publish_transform(&self, id: TransformDraftId) -> Result<DbTransform, ServiceError>;
   
    async fn delete_transform(&self, id: TransformId) -> Result<(), ServiceError>;

    async fn delete_transform_draft(&self,id:TransformDraftId)->Result<(),ServiceError>;
}
