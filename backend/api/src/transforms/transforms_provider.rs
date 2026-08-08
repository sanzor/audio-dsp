use domain::db::{
    db_transform::{DbTransform, DbTransformBinary, DbTransformDefinition, TransformId},
    ticket::db_resource::ResourceId,
    transform_snapshot::CompositeTransformDefinition,
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
/// (bucket 2 — `transform_draft`) against what's currently live
/// (bucket 3 — `transform_port`). Purely advisory: nothing in the backend
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
    /// `kind` is "primitive" | "composite" — validated by the controller.
    async fn create_transform(&self, name: String, description: Option<String>, icon: Option<String>, kind: String) -> Result<DbTransformDefinition, ServiceError>;
    /// Bucket 2 — save. Always overwrites source_code; if `resource_id` is
    /// given, also attaches that compile resource's binary/metadata.
    async fn save_transform_draft(&self, id: TransformId, source_code: String, resource_id: Option<ResourceId>) -> Result<DbTransformDefinition, ServiceError>;
    /// Composite counterpart to `save_transform_draft` — persists the working
    /// wiring graph as-is, structurally, with no validation and no derived
    /// `ports` write (whatever `ports` already held is left untouched).
    /// Always flips `is_validated` back to `false` — see
    /// `validate_composite_draft` below and
    /// `agents/decisions/0007-composite-draft-validation-gate.md`.
    async fn save_composite_draft(&self, id: TransformId, graph: CompositeTransformDefinition) -> Result<DbTransformDefinition, ServiceError>;
    /// New explicit validate action for a composite draft. Runs
    /// `composite_validator::validate_composite_graph` (every referenced
    /// transform exists, is a published primitive, ports/cardinality/
    /// exposure rules hold) against the *currently-persisted*
    /// `graph_definition`. On success, writes the derived `ports` and flips
    /// `is_validated` to `true`. On failure, leaves `ports`/`is_validated`
    /// untouched and returns the validation error. Independent of Publish —
    /// Publish keeps re-running the same validation from scratch and never
    /// checks `is_validated` as a precondition. See
    /// `agents/decisions/0007-composite-draft-validation-gate.md`.
    async fn validate_composite_draft(&self, id: TransformId) -> Result<DbTransformDefinition, ServiceError>;
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
