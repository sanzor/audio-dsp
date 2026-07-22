use std::sync::Arc;

use domain::db::ticket::{create_ticket_params::CreateTicketParams, db_resource::ResourceId, db_ticket::{DbTicket, TicketId}};

use crate::{
    domain::service_error::ServiceError,
    infra::producer::producer::Producer,
    tickets::{compile_params::RequestCompileParams, compile_result::CompileResult},
    transforms::data_provider::transforms_data_provider::TransformsDataProvider,
    ticket_worker::events::ticket_created_event::TicketCreatedEvent,
};

use super::tickets_provider::TicketsProvider;

pub struct TicketsProviderService {
    data: Arc<dyn TransformsDataProvider>,
    producer: Arc<dyn Producer<TicketCreatedEvent>>,
}

impl TicketsProviderService {
    pub fn new(data: Arc<dyn TransformsDataProvider>, producer: Arc<dyn Producer<TicketCreatedEvent>>) -> Self {
        Self { data, producer }
    }
}

#[async_trait::async_trait]
impl TicketsProvider for TicketsProviderService {
    async fn request_compile_transform(&self, params: RequestCompileParams) -> Result<DbTicket, ServiceError> {
        let source_code = params.payload;
        let transform_id = params.transform_id;

        let ticket = self.data.create_ticket(CreateTicketParams {
            transform_id,
            user_id: params.user_id,
            source_code: source_code.clone(),
        }).await.map_err(ServiceError::from)?;

        if let Err(e) = self.producer.produce(TicketCreatedEvent {
            ticket_id: ticket.id,
            transform_id,
            source_code,
        }).await {
            tracing::error!(error = %e, "failed to produce TicketCreatedEvent");
        }

        Ok(ticket)
    }

    async fn get_compile_ticket_status(&self, id: TicketId) -> Result<DbTicket, ServiceError> {
        self.data.get_ticket(id)
            .await
            .map_err(ServiceError::from)
    }

    async fn get_ticket_result(&self, id: ResourceId) -> Result<CompileResult, ServiceError> {
        let resource = self.data.get_resource(id).await.map_err(ServiceError::from)?;
        Ok(CompileResult {
            resource_id: resource.id,
            ticket_id: resource.ticket_id,
            wasm_bytecode: resource.wasm_bytecode,
            params: resource.params,
        })
    }
}
