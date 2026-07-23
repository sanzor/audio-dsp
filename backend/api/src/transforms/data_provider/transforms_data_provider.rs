use domain::db::{
    db_transform::{DbTransform, DbTransformDefinition, DbTransformPort, TransformId},
    ticket::{create_ticket_params::CreateTicketParams, db_resource::{DbResource, ResourceId}, db_ticket::{DbTicket, TicketId}, update_ticket_params::UpdateTicketParams},
    transform_saved_state::DbTransformSavedState,
    transform_snapshot::{ParamSnapshot, PortSnapshot},
};

use crate::{domain::data_error::DataError};

/// A port as introspected from a compiled transform's metadata, not yet
/// persisted. `direction`/`kind`/`cardinality` are already validated/mapped
/// to their DB string forms by the caller (see
/// worker::processor::metadata_introspector).
pub struct NewTransformPort {
    pub name: String,
    pub direction: String,
    pub order: i32,
    pub description: Option<String>,
    pub kind: String,
    pub cardinality: String,
}

/// A param as introspected from a compiled transform's metadata, not yet
/// persisted.
pub struct NewTransformParam {
    pub name: String,
    pub order: i32,
    pub default_value: f32,
    pub min_value: Option<f32>,
    pub max_value: Option<f32>,
    pub description: Option<String>,
}

impl From<NewTransformPort> for PortSnapshot {
    fn from(p: NewTransformPort) -> Self {
        Self {
            name: p.name,
            direction: p.direction,
            port_order: p.order,
            description: p.description,
            kind: p.kind,
            cardinality: p.cardinality,
        }
    }
}

impl From<PortSnapshot> for NewTransformPort {
    fn from(p: PortSnapshot) -> Self {
        Self {
            name: p.name,
            direction: p.direction,
            order: p.port_order,
            description: p.description,
            kind: p.kind,
            cardinality: p.cardinality,
        }
    }
}

impl From<NewTransformParam> for ParamSnapshot {
    fn from(p: NewTransformParam) -> Self {
        Self { name: p.name, param_order: p.order, default_value: p.default_value, min_value: p.min_value, max_value: p.max_value, description: p.description }
    }
}

impl From<ParamSnapshot> for NewTransformParam {
    fn from(p: ParamSnapshot) -> Self {
        Self { name: p.name, order: p.param_order, default_value: p.default_value, min_value: p.min_value, max_value: p.max_value, description: p.description }
    }
}

#[async_trait::async_trait]
pub trait TransformsDataProvider: Send + Sync {

    async fn create_ticket(&self,ticket:CreateTicketParams)->Result<DbTicket,DataError>;
    async fn get_ticket(&self,ticket_id:TicketId)->Result<DbTicket,DataError>;

    /// Stores the full artifact a successful compile ticket produced —
    /// bucket 1. Immutable history; never touches bucket 2 (save) or
    /// bucket 3 (published) state.
    async fn create_resource(
        &self,
        ticket_id: TicketId,
        wasm_bytecode: Vec<u8>,
        name: String,
        description: Option<String>,
        ports: Vec<NewTransformPort>,
        params: Vec<NewTransformParam>,
    ) -> Result<DbResource, DataError>;
    async fn get_resource(&self,resource_id:ResourceId)->Result<DbResource,DataError>;
    async fn update_resource(&self, resource_id: ResourceId, ticket_id: TicketId) -> Result<DbResource, DataError>;
    async fn remove_resource(&self, resource_id: ResourceId) -> Result<(), DataError>;
    async fn remove_ticket(&self,ticket_id:TicketId)->Result<(),DataError>;
    async fn update_ticket(&self,ticket:UpdateTicketParams)->Result<DbTicket,DataError>;
    
    async fn get_transform(&self, id: TransformId) -> Result<DbTransform, String>;
    async fn get_transform_definition(&self, id: TransformId) -> Result<DbTransformDefinition, String>;
    /// Currently-published (bucket 3) ports only — no join through saved
    /// state. Empty for a transform that's never been published. Used by
    /// the pre-publish port-shape diff (see
    /// `TransformsProvider::diff_publish_port_shape`) to compare against
    /// what's about to be published.
    async fn get_current_ports(&self, transform_id: TransformId) -> Result<Vec<DbTransformPort>, DataError>;
    async fn get_transform_definitions(&self, ids: &[TransformId]) -> Result<Vec<DbTransformDefinition>, String>;
    async fn list_transform_summaries(&self, offset: i64, limit: i64) -> Result<(Vec<DbTransform>, i64), String>;
    /// Also creates the transform's (bucket 2) saved-state row, so it's
    /// always present — save/publish never have to special-case "no row yet".
    async fn insert_transform(&self, name: String, description: Option<String>, icon: Option<String>) -> Result<DbTransform, String>;
    /// Only allowed when the transform has never been published (no row in
    /// `transform_binaries`) — see `agents/decisions/0002-transform-draft-lifecycle-decisions.md`.
    /// Cascades to `transform_saved_state`/`transform_tickets`/`transform_resources`
    /// via existing FK `ON DELETE CASCADE`.
    async fn delete_transform(&self, id: TransformId) -> Result<(), DataError>;

    async fn get_saved_state(&self, transform_id: TransformId) -> Result<DbTransformSavedState, DataError>;
    /// Bucket 2 — "save". Always overwrites source_code. If `resource_id` is
    /// given, also copies that resource's (bucket 1) binary/metadata into the
    /// saved state; the resource must belong to this transform. If omitted,
    /// any previously saved binary/metadata is left untouched — a source-only
    /// save never wipes out the last good build.
    async fn save_transform_state(
        &self,
        transform_id: TransformId,
        source_code: String,
        resource_id: Option<ResourceId>,
    ) -> Result<DbTransformSavedState, DataError>;

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
