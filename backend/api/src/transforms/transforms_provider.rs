use domain::{db::{
    WorkspaceId, db_transform::{DbTransform, TransformId},
     db_transform_draft::{DbTransformDraft, TransformDraftId}, ticket::db_resource::ResourceId,
}, domain_user::UserId};

use crate::domain::service_error::ServiceError;

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
    
    async fn validate_composite_draft(&self, id: TransformDraftId) -> Result<bool, ServiceError>;
    /// Bucket 3 — publish. Bundles whatever's currently saved (bucket 2) into
    /// the live artifact. Fails if nothing has been saved with a binary yet.
    async fn publish_transform(&self, id: TransformDraftId) -> Result<DbTransform, ServiceError>;
   
    async fn delete_transform(&self, id: TransformId) -> Result<(), ServiceError>;

    async fn delete_transform_draft(&self,id:TransformDraftId)->Result<(),ServiceError>;
}
