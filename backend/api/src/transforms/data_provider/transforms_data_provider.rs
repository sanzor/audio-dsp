use domain::db::{db_transform::{DbTransform, DbTransformDefinition, DbTransformPort, TransformId}, ticket::{create_ticket_params::CreateTicketParams, db_resource::{DbResource, ResourceId}, db_ticket::{DbTicket, TicketId}, update_ticket_params::UpdateTicketParams}};

use crate::{domain::data_error::DataError};

/// A port as introspected from a compiled transform's metadata, not yet
/// persisted. `direction` is already validated to be "input"/"output" by
/// the caller (see worker::processor::metadata_introspector).
pub struct NewTransformPort {
    pub name: String,
    pub direction: String,
    pub order: i32,
    pub description: Option<String>,
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

#[async_trait::async_trait]
pub trait TransformsDataProvider: Send + Sync {

    async fn create_ticket(&self,ticket:CreateTicketParams)->Result<DbTicket,DataError>;
    async fn get_ticket(&self,ticket_id:TicketId)->Result<DbTicket,DataError>;

    async fn create_resource(&self, ticket_id: TicketId) -> Result<DbResource, DataError>;
    async fn get_resource(&self,resource_id:ResourceId)->Result<DbResource,DataError>;
    async fn update_resource(&self, resource_id: ResourceId, ticket_id: TicketId) -> Result<DbResource, DataError>;
    async fn remove_resource(&self, resource_id: ResourceId) -> Result<(), DataError>;
    async fn remove_ticket(&self,ticket_id:TicketId)->Result<(),DataError>;
    async fn update_ticket(&self,ticket:UpdateTicketParams)->Result<DbTicket,DataError>;
    
    async fn get_transform(&self, id: TransformId) -> Result<DbTransform, String>;
    async fn get_transform_definition(&self, id: TransformId) -> Result<DbTransformDefinition, String>;
    async fn get_transform_definitions(&self, ids: &[TransformId]) -> Result<Vec<DbTransformDefinition>, String>;
    async fn list_transform_summaries(&self, offset: i64, limit: i64) -> Result<(Vec<DbTransform>, i64), String>;
    async fn insert_transform(&self, name: String, description: Option<String>, icon: Option<String>) -> Result<DbTransform, String>;
    async fn update_transform(&self, id: TransformId, name: String, description: Option<String>, icon: Option<String>) -> Result<DbTransform, String>;
    async fn delete_transform(&self, id: TransformId) -> Result<(), String>;
    async fn insert_port(&self, transform_id: TransformId, name: String, direction: String, port_order: i32, description: Option<String>) -> Result<DbTransformPort, String>;
    async fn delete_port(&self, port_id: i64) -> Result<(), String>;

    /// Atomically replaces a transform's ports/params with the set derived
    /// from a successful compile, and publishes the compiled binary as the
    /// live artifact. One transaction so a transform's definition and its
    /// binary can never observably drift from each other.
    async fn publish_compiled_transform(
        &self,
        transform_id: TransformId,
        wasm_bytecode: Vec<u8>,
        source_code: String,
        ports: Vec<NewTransformPort>,
        params: Vec<NewTransformParam>,
    ) -> Result<(), String>;
}
