use std::sync::Arc;

use domain::db::{
    db_transform::TransformId,
    ticket::{
        create_ticket_params::CreateTicketParams,
        db_resource::ResourceId,
        db_ticket::{DbTicket, TicketId},
    },
};

use crate::{
    domain::service_error::ServiceError,
    infra::producer::producer::Producer,
    ticket_worker::events::ticket_created_event::TicketCreatedEvent,
    tickets::{compile_params::RequestCompileParams, compile_result::CompileResult},
    transforms::data_provider::transforms_data_provider::TransformsDataProvider,
};

use super::tickets_provider::TicketsProvider;

pub struct TicketsProviderService {
    data: Arc<dyn TransformsDataProvider>,
    producer: Arc<dyn Producer<TicketCreatedEvent>>,
}

impl TicketsProviderService {
    pub fn new(
        data: Arc<dyn TransformsDataProvider>,
        producer: Arc<dyn Producer<TicketCreatedEvent>>,
    ) -> Self {
        Self { data, producer }
    }
}

#[async_trait::async_trait]
impl TicketsProvider for TicketsProviderService {
    async fn request_compile_transform(
        &self,
        params: RequestCompileParams,
    ) -> Result<DbTicket, ServiceError> {
        let source_code = params.payload;
        let transform_id = params.transform_id;

        let ticket = self
            .data
            .create_transform_ticket(CreateTicketParams {
                transform_id,
                user_id: params.user_id,
                source_code: source_code.clone(),
            })
            .await
            .map_err(ServiceError::from)?;

        if let Err(e) = self
            .producer
            .produce(TicketCreatedEvent {
                ticket_id: ticket.id,
                transform_id,
                source_code,
            })
            .await
        {
            tracing::error!(error = %e, "failed to produce TicketCreatedEvent");
        }

        Ok(ticket)
    }

    async fn get_compile_ticket_status(&self, id: TicketId) -> Result<DbTicket, ServiceError> {
        self.data.get_ticket(id).await.map_err(ServiceError::from)
    }

    async fn get_ticket_result(
        &self,
        resource_id: ResourceId,
    ) -> Result<CompileResult, ServiceError> {
        let resource = self
            .data
            .get_compiled_transform(resource_id)
            .await
            .map_err(ServiceError::from)?;
        Ok(CompileResult {
            resource_id: resource.id,
            ticket_id: resource.ticket_id,
            source_code: resource.source_code.ok_or_else(|| {
                ServiceError::Internal("compile resource is missing its ticket source".to_string())
            })?,
            wasm_bytecode: resource.wasm_bytecode,
        })
    }

    async fn get_ticket_transform_id(
        &self,
        ticket_id: TicketId,
    ) -> Result<TransformId, ServiceError> {
        self.data
            .get_ticket_transform_id(ticket_id)
            .await
            .map_err(ServiceError::from)
    }

    async fn get_compiled_transform_id(
        &self,
        resource_id: ResourceId,
    ) -> Result<TransformId, ServiceError> {
        self.data
            .get_resource_transform_id(resource_id)
            .await
            .map_err(ServiceError::from)
    }
}
