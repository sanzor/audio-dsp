use domain::db::ticket::{db_resource::ResourceId, db_ticket::{DbTicket, TicketId}};

use crate::{domain::service_error::ServiceError, tickets::{compile_params::RequestCompileParams, compile_result::CompileResult}};

#[async_trait::async_trait]
pub trait TicketsProvider: Send + Sync {
    async fn request_compile_transform(&self, params: RequestCompileParams) -> Result<DbTicket, ServiceError>;
    async fn get_compile_ticket_status(&self, ticket_id: TicketId) -> Result<DbTicket, ServiceError>;
    async fn get_ticket_result(&self, id: ResourceId) -> Result<CompileResult, ServiceError>;
}
