use std::sync::Arc;

use domain::db::ticket::{ticket_status::TicketStatus, update_ticket_params::UpdateTicketParams};

use crate::transforms::{
    data_provider::transforms_data_provider::TransformsDataProvider,
    storage_provider::transform_storage_provider::TransformStorageProvider,
};

use super::{
    process_params::ProcessParams,
    process_result::ProcessResult,
    processor_error::ProcessorError,
    processor_params::ProcessorParams,
};

pub struct Processor {
    data_provider: Arc<dyn TransformsDataProvider>,
    storage: Box<dyn TransformStorageProvider>,
}

impl Processor {
    pub fn new(params: ProcessorParams) -> Self {
        Self {
            data_provider: params.data_provider,
            storage: params.storage,
        }
    }

    pub async fn process(&self, params: ProcessParams) -> Result<ProcessResult, ProcessorError> {
        let event = &params.event;

        // 1 — verify ticket exists
        self.data_provider
            .get_ticket(event.ticket_id)
            .await
            .map_err(|e| ProcessorError::DataError(e.to_string()))?;

        // 2 — compile WAT → WASM binary (CPU-bound, offloaded to blocking pool)
        let binary = match Self::compile_source_code(&event.source_code).await {
            Ok(b) => b,
            Err(e) => {
                self.mark_failed(event.ticket_id, e.clone()).await;
                return Err(ProcessorError::CompileError(e));
            }
        };

        // 3 — persist binary (UPSERT by transform_id)
        if let Err(e) = self.storage
            .write_transform_binary(event.transform_id, &binary)
            .await
        {
            self.mark_failed(event.ticket_id, e.clone()).await;
            return Err(ProcessorError::WriteError(e));
        }

        // 4 — create resource record and mark ticket successful
        let resource = self.data_provider
            .create_resource(event.ticket_id)
            .await
            .map_err(|e| ProcessorError::DataError(e.to_string()))?;

        let ticket = self.data_provider
            .update_ticket(UpdateTicketParams {
                ticket_id: event.ticket_id,
                status: TicketStatus::Successful { resource_id: resource.id },
            })
            .await
            .map_err(|e| ProcessorError::DataError(e.to_string()))?;

        Ok(ProcessResult { ticket })
    }

    /// Compile WAT text source to WASM binary.
    /// Runs on the blocking thread pool because `wat::parse_str` is synchronous
    /// and can be slow for large inputs.
    async fn compile_source_code(source_code: &str) -> Result<Vec<u8>, String> {
        let source = source_code.to_owned();
        tokio::task::spawn_blocking(move || {
            wat::parse_str(&source).map_err(|e| e.to_string())
        })
        .await
        .map_err(|e| e.to_string())?
    }

    /// Best-effort: update ticket to Failed. Errors here are logged but not
    /// propagated so the original error is preserved for the caller.
    async fn mark_failed(&self, ticket_id: domain::db::ticket::db_ticket::TicketId, message: String) {
        if let Err(e) = self.data_provider
            .update_ticket(UpdateTicketParams {
                ticket_id,
                status: TicketStatus::Failed { message },
            })
            .await
        {
            tracing::error!(
                ticket_id,
                error = %e,
                "failed to mark ticket as failed after processing error"
            );
        }
    }
}
