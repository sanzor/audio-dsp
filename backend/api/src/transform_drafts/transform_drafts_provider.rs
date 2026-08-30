use domain::{db::{db_transform::DbTransform, db_transform_draft::{DbTransformDraft, TransformDraftId}}, domain_user::UserId};

use crate::domain::service_error::ServiceError;
use crate::ticket_worker::processor::transform_metadata::PortMetadataJson;

#[async_trait::async_trait]
pub trait TransformDraftsProvider: Send + Sync {
    async fn get_transform_draft(&self, id: TransformDraftId) -> Result<DbTransformDraft, ServiceError>;
    async fn get_transform_drafts(&self, ids: &[TransformDraftId]) -> Result<Vec<DbTransformDraft>, ServiceError>;
    /// Cheap point-lookup for ownership checks.
    async fn get_transform_draft_owner(&self, id: TransformDraftId) -> Result<UserId, ServiceError>;
    /// `kind` is "primitive" | "composite" — validated by the controller.
    async fn create_transform_draft(&self,
        name: String,
        description: Option<String>,
        icon: Option<String>,
        kind: String,
        owner_user_id: UserId) -> Result<DbTransformDraft, ServiceError>;
    /// Bucket 2 — save, primitive only. Errors if `id` is a composite draft.
    /// An optional frontend-held compiled WASM payload is saved atomically
    /// with source after server-side introspection.
    async fn save_primitive_draft(&self,
         id: TransformDraftId,
         source_code: String,
         wasm_bytecode: Option<Vec<u8>>) -> Result<DbTransformDraft, ServiceError>;
    /// Bucket 2 — save, composite only. Errors if `id` is a primitive draft.
    /// `graph_json` is the raw wiring graph (`{nodes, edges}`, same shape
    /// `validate_graph_draft` takes) — stored as-is; the richer
    /// `{name, description, ports, params, graph}` envelope only gets built
    /// at `publish_composite` time, since nothing reads a draft's ports.
    async fn save_composite_draft(&self, id: TransformDraftId, graph_json: String) -> Result<DbTransformDraft, ServiceError>;

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
    async fn validate_graph_draft(&self, id: TransformDraftId, graph_json: String) -> Result<Vec<PortMetadataJson>, ServiceError>;
    /// Bucket 3 — publish, primitive only. Errors if `id` is a composite
    /// draft. Bundles whatever's currently saved (bucket 2) into the live
    /// artifact; fails if nothing has been saved with a binary yet.
    async fn publish_primitive(&self, id: TransformDraftId) -> Result<DbTransform, ServiceError>;
    /// Bucket 3 — publish, composite only. Errors if `id` is a primitive
    /// draft. Re-validates the currently-saved graph (cheap — no ticket
    /// involved) and publishes the derived ports alongside it; fails if
    /// nothing has been saved yet, or the saved graph no longer validates
    /// (e.g. a referenced transform was deleted since the last save).
    async fn publish_composite(&self, id: TransformDraftId) -> Result<DbTransform, ServiceError>;

    async fn delete_transform_draft(&self, id: TransformDraftId) -> Result<(), ServiceError>;
}
