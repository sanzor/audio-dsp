use domain::db::{
    db_transform::{DbTransform, DbTransformBinary, DbTransformDefinition, TransformId},
    ticket::db_resource::ResourceId,
};

use crate::domain::service_error::ServiceError;

/// One port's shape, for the pre-publish diff below — deliberately narrower
/// than the full port record (no id/order/description), since only
/// count/kind/cardinality per direction is what a shape change means here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortShapeSummary {
    pub name: String,
    pub direction: String,
    pub kind: String,
    pub cardinality: String,
}

/// Result of comparing a transform's about-to-be-published port shape
/// (bucket 2 — `transform_saved_state`) against what's currently live
/// (bucket 3 — `transform_ports`). Purely advisory: nothing in the backend
/// blocks on `changed` — it exists so the creator's Publish flow can show a
/// non-blocking confirm dialog before proceeding. `changed` is always
/// `false` when the transform has never been published before (nothing to
/// compare against yet, so nothing to warn about).
#[derive(Debug, Clone)]
pub struct PublishPortShapeDiff {
    pub changed: bool,
    pub current: Vec<PortShapeSummary>,
    pub incoming: Vec<PortShapeSummary>,
}

#[async_trait::async_trait]
pub trait TransformsProvider: Send + Sync {
    async fn list_transform_summaries(&self, offset: i64, limit: i64) -> Result<(Vec<DbTransform>, i64), ServiceError>;
    async fn get_transform_definition(&self, id: TransformId) -> Result<DbTransformDefinition, ServiceError>;
    async fn get_transform_definitions(&self, ids: &[TransformId]) -> Result<Vec<DbTransformDefinition>, ServiceError>;
    async fn get_transform_binary(&self, id: TransformId) -> Result<Vec<u8>, ServiceError>;
    async fn get_transform_binaries(&self, ids: &[TransformId]) -> Result<Vec<DbTransformBinary>, ServiceError>;
    async fn create_transform(&self, name: String, description: Option<String>, icon: Option<String>) -> Result<DbTransformDefinition, ServiceError>;
    /// Bucket 2 — save. Always overwrites source_code; if `resource_id` is
    /// given, also attaches that compile resource's binary/metadata.
    async fn save_transform_state(&self, id: TransformId, source_code: String, resource_id: Option<ResourceId>) -> Result<DbTransformDefinition, ServiceError>;
    /// Bucket 3 — publish. Bundles whatever's currently saved (bucket 2) into
    /// the live artifact. Fails if nothing has been saved with a binary yet.
    async fn publish_transform(&self, id: TransformId) -> Result<DbTransformDefinition, ServiceError>;
    /// Advisory pre-publish check — does NOT mutate anything and is never
    /// called by `publish_transform` itself. Meant to be polled by the
    /// creator's Publish flow immediately before calling
    /// `publish_transform`, so it can show a non-blocking confirm dialog
    /// when the about-to-be-published port shape differs from what's
    /// currently live.
    async fn diff_publish_port_shape(&self, id: TransformId) -> Result<PublishPortShapeDiff, ServiceError>;
    async fn delete_transform(&self, id: TransformId) -> Result<(), ServiceError>;
}
