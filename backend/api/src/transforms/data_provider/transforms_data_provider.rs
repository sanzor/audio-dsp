use domain::db::{db_transform::{DbTransform, DbTransformDefinition, DbTransformPort, TransformId}, ticket::{create_ticket_params::CreateTicketParams, db_ticket::DbTicket}};

use crate::{domain::data_error::DataError, transforms::ticket::TicketId};

#[async_trait::async_trait]
pub trait TransformsDataProvider: Send + Sync {

    async fn create_ticket(&self,ticket:CreateTicketParams)->Result<DbTicket,DataError>;
    async fn get_ticket(&self,ticket_id:TicketId)->Result<DbTicket,DataError>;
    async fn remove_ticket(&self,ticket_id:TicketId)->Result<(),DataError>;
    async fn update_ticket(&self,ticket_id:TicketId)->Result<DbTicket,DataError>;
    
    async fn get_transform(&self, id: TransformId) -> Result<DbTransform, String>;
    async fn get_transform_definition(&self, id: TransformId) -> Result<DbTransformDefinition, String>;
    async fn get_transform_definitions(&self, ids: &[TransformId]) -> Result<Vec<DbTransformDefinition>, String>;
    async fn list_transform_summaries(&self, offset: i64, limit: i64) -> Result<(Vec<DbTransform>, i64), String>;
    async fn insert_transform(&self, name: String, description: Option<String>, icon: Option<String>) -> Result<DbTransform, String>;
    async fn update_transform(&self, id: TransformId, name: String, description: Option<String>, icon: Option<String>) -> Result<DbTransform, String>;
    async fn delete_transform(&self, id: TransformId) -> Result<(), String>;
    async fn insert_port(&self, transform_id: TransformId, name: String, direction: String, port_order: i32, description: Option<String>) -> Result<DbTransformPort, String>;
    async fn delete_port(&self, port_id: i64) -> Result<(), String>;
}
