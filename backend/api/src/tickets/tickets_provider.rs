use domain::db::{
    db_transform::TransformId,
    ticket::{
        db_resource::ResourceId,
        db_ticket::{DbTicket, TicketId},
    },
};

use crate::{
    domain::service_error::ServiceError,
    tickets::{compile_params::RequestCompileParams, compile_result::CompileResult},
};

#[async_trait::async_trait]
pub trait TicketsProvider: Send + Sync {
    async fn request_compile_transform(
        &self,
        params: RequestCompileParams,
    ) -> Result<DbTicket, ServiceError>;
    async fn get_compile_ticket_status(
        &self,
        ticket_id: TicketId,
    ) -> Result<DbTicket, ServiceError>;
    async fn get_ticket_result(
        &self,
        resource_id: ResourceId,
    ) -> Result<CompileResult, ServiceError>;
    /// Point-lookups used for authorization before fetching the full ticket/resource.
    async fn get_ticket_transform_id(
        &self,
        ticket_id: TicketId,
    ) -> Result<TransformId, ServiceError>;
    async fn get_compiled_transform_id(
        &self,
        resource_id: ResourceId,
    ) -> Result<TransformId, ServiceError>;
}
