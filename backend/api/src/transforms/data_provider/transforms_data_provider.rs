use std::collections::HashMap;

use domain::{db::{
    WorkspaceId, db_transform::{DbTransform, TransformId}, db_transform_draft::DbTransformDraft, ticket::{create_ticket_params::CreateTicketParams, db_resource::{DbResource, ResourceId}, db_ticket::{DbTicket, TicketId}, update_ticket_params::UpdateTicketParams}, transform_snapshot::{CompositeTransformDefinition, ParamSnapshot, PortSnapshot},
}, domain_user::UserId, user::User};

use crate::transforms::composite_validator::LeafTransformInfo;

use crate::{domain::data_error::DataError};


#[async_trait::async_trait]
pub trait TransformsDataProvider: Send + Sync {

    async fn create_transform_ticket(&self,ticket:CreateTicketParams)->Result<DbTicket,DataError>;
    async fn get_ticket(&self,ticket_id:TicketId)->Result<DbTicket,DataError>;
    /// Point-lookup used for authorization — resolves which transform a
    /// ticket/resource belongs to without fetching the full ticket/resource.
    async fn get_ticket_transform_id(&self, ticket_id: TicketId) -> Result<TransformId, DataError>;
    async fn get_resource_transform_id(&self, resource_id: ResourceId) -> Result<TransformId, DataError>;

    /// Stores the full artifact a successful compile ticket produced —
    /// bucket 1. Immutable history; never touches bucket 2 (save) or
    /// bucket 3 (published) state.
    async fn create_resource(
        &self,
        ticket_id: TicketId,
        wasm_bytecode: Vec<u8>,
        name: String,
        description: Option<String>,
        metadata:String
    ) -> Result<DbResource, DataError>;
    async fn get_compiled_transform(&self,resource_id:ResourceId)->Result<DbResource,DataError>;
    async fn update_compiled_transform(&self, resource_id: ResourceId, ticket_id: TicketId) -> Result<DbResource, DataError>;
    async fn remove_compiled_transform(&self, resource_id: ResourceId) -> Result<(), DataError>;
    async fn remove_ticket(&self,ticket_id:TicketId)->Result<(),DataError>;
    async fn update_ticket(&self,ticket:UpdateTicketParams)->Result<DbTicket,DataError>;
    
    async fn get_transform(&self, id: TransformId) -> Result<DbTransform, String>;
    async fn get_transform_owner(&self, id: TransformId) -> Result<User, String>;
    
    async fn get_transforms(&self, offset: i64, limit: i64) -> Result<(Vec<DbTransform>, i64), String>;
    /// Catalog for one workspace: transforms owned by `user_id`, granted
    /// directly to `user_id`, or granted to `workspace_id` itself.
    async fn get_transforms_for_workspace_and_user(&self, user_id: UserId, workspace_id: WorkspaceId) -> Result<Vec<DbTransform>, String>;
    /// Also creates the transform's (bucket 2) draft row, so it's
    /// always present — save/publish never have to special-case "no row yet".
    /// `kind` is "primitive" | "composite", validated by the caller.
    async fn insert_transform(&self, 
        name: String, 
        description: Option<String>, 
        icon: Option<String>, 
        kind: String, 
        owner_user_id: i32,
        source_code:String,
        metadata:String,
        wasm_bytecode: Vec<u8>) -> Result<DbTransform, String>;

    /// Bucket 2 — "save" for a composite. Writes the working graph_definition
    /// as-is (mirrors save_transform_draft's role for primitives, but there's
    /// no source_code/wasm to preserve). Unconditionally flips
    /// `is_validated` back to `false` — any graph edit invalidates the last
    /// validate result. Does **not** touch `ports`; the previously-derived
    /// set (from the last successful validate/publish, or empty for a
    /// brand-new draft) is left exactly as-is. See
    /// `agents/decisions/0007-composite-draft-validation-gate.md`.
    async fn save_transform_draft(
        &self,
        transform_id: TransformId,
        metadata: String,
    ) -> Result<DbTransformDraft, DataError>;

    /// New explicit validate action for a composite draft — writes the
    /// already-validated derived ports (computed by the caller via
    /// `composite_validator::validate_composite_graph` against the
    /// currently-persisted `graph_definition`) and flips `is_validated` to
    /// `true`. Only ever called after that validation has already succeeded;
    /// this method itself does no validation, it just persists the result.
    /// See `agents/decisions/0007-composite-draft-validation-gate.md`.
    async fn validate_transform(
        &self,
        transform_id: TransformId,
    ) -> Result<DbTransformDraft, DataError>;

    /// Composite counterpart to `publish_compiled_transform` — same
    /// transaction shape (name/description update, delete+reinsert
    /// transform_port), but promotes graph_definition into transform_composite
    /// instead of a compiled binary into transform_binary. No params rows
    /// (v1 composites always have params: []).
    async fn publish_transform(
        &self,
        transform_id: TransformId,
        name: String,
        description: Option<String>
    ) -> Result<(), String>;
    /// Only allowed when the transform has never been published (no row in
    /// `transform_binary`) — see `agents/decisions/0002-transform-draft-lifecycle-decisions.md`.
    /// Cascades to `transform_draft`/`transform_ticket`/`transform_resource`
    /// via existing FK `ON DELETE CASCADE`.
    async fn delete_transform(&self, id: TransformId) -> Result<(), DataError>;

    async fn get_draft(&self, transform_id: TransformId) -> Result<DbTransformDraft, DataError>;
    /// Bucket 2 — "save". Always overwrites source_code. If `resource_id` is
    /// given, also copies that resource's (bucket 1) binary/metadata into the
    /// draft; the resource must belong to this transform. If omitted,
    /// any previously saved binary/metadata is left untouched — a source-only
    /// save never wipes out the last good build.
    async fn save_transform_draft(
        &self,
        transform_id: TransformId,
        source_code: String,
        resource_id: Option<ResourceId>,
    ) -> Result<DbTransformDraft, DataError>;

    /// Atomically replaces a transform's name/description/ports/params with
    /// the set derived from a successful compile, and publishes the compiled
    /// binary as the live artifact. One transaction so a transform's
    /// definition and its binary can never observably drift from each other.
    async fn publish_compiled_transform(
        &self,
        transform_id: TransformId,
        wasm_bytecode: Vec<u8>,
        source_code: String,
        name: String,
        description: Option<String>,
        ports: Vec<NewTransformPort>,
        params: Vec<NewTransformParam>,
    ) -> Result<(), String>;
}
