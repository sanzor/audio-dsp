use domain::db::db_transform::{
    DbTransform, DbTransformBinary, DbTransformDefinition, DbTransformPort, TransformId,
};

use crate::transforms::{compile_params::RequestCompileParams, compile_result::CompileResult, request_compile_result::RequestCompileResult, ticket::{ResourceId, TicketId, TicketStatusResult}};

#[async_trait::async_trait]
pub trait TransformsProvider: Send + Sync {
    async fn request_compile_transform(&self,params:RequestCompileParams)->Result<RequestCompileResult,String>;
    async fn get_compile_ticket_status(&self,ticket_id:TicketId)->Result<TicketStatusResult,String>;
    async fn get_ticket_result(&self,id:ResourceId)->Result<CompileResult,String>;
    async fn list_transform_summaries(&self, offset: i64, limit: i64) -> Result<(Vec<DbTransform>, i64), String>;
    async fn get_transform_definition(&self, id: TransformId) -> Result<DbTransformDefinition, String>;
    async fn get_transform_definitions(&self, ids: &[TransformId]) -> Result<Vec<DbTransformDefinition>, String>;
    async fn get_transform_binary(&self, id: TransformId) -> Result<Vec<u8>, String>;
    async fn get_transform_binaries(&self, ids: &[TransformId]) -> Result<Vec<DbTransformBinary>, String>;
    async fn create_transform(&self, name: String, description: Option<String>, icon: Option<String>) -> Result<DbTransformDefinition, String>;
    async fn update_transform(&self, id: TransformId, name: String, description: Option<String>, icon: Option<String>) -> Result<DbTransformDefinition, String>;
    async fn delete_transform(&self, id: TransformId) -> Result<(), String>;
    async fn add_port(&self, transform_id: TransformId, name: String, direction: String, port_order: i32, description: Option<String>) -> Result<DbTransformPort, String>;
    async fn delete_port(&self, port_id: i64) -> Result<(), String>;

}
