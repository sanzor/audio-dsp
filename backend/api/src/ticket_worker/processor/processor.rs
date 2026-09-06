use std::sync::Arc;

use domain::db::ticket::{ticket_status::TicketStatus, update_ticket_params::UpdateTicketParams};

use crate::{
    ticket_worker::processor::{build_job_config::BuildJobConfig, validator, wasm::wasm_parser::{self, WasmInput}}, transforms::data_provider::transforms_data_provider::TransformsDataProvider,
};

use super::{
    build_job::{self},
    process_params::ProcessParams,
    process_result::ProcessResult,
    processor_error::ProcessorError,
    processor_params::ProcessorParams,
};

pub struct Processor {
    data_provider: Arc<dyn TransformsDataProvider>,
    build_job_config: BuildJobConfig,
    metadata_fuel_limit: u64,
}

impl Processor {
    pub fn new(params: ProcessorParams) -> Self {
        Self {
            data_provider: params.data_provider,
            build_job_config: params.build_job_config,
            metadata_fuel_limit: params.metadata_fuel_limit,
        }
    }

    pub async fn process(&self, params: ProcessParams) -> Result<ProcessResult, ProcessorError> {
        let event = &params.event;

        // 1 — verify ticket exists
        if self
            .data_provider
            .get_ticket(event.ticket_id)
            .await
            .is_err()
        {
            return Err(ProcessorError::DataError(format!(
                "Could not find ticket with id {:?}",
                event.ticket_id
            )));
        }

        // 2 — compile the submitted Rust source to wasm32-unknown-unknown
        let wasm_bytecode = match build_job::compile_transform_source(
            &self.build_job_config,
            event.ticket_id,
            &event.source_code,
        )
        .await
        {
            Ok(bytes) => bytes,
            Err(e) => {
                self.mark_failed(event.ticket_id, e.clone()).await;
                return Err(ProcessorError::CompileError(e));
            }
        };
        let metadata=wasm_parser::parse_wasm(WasmInput{
            fuel_limit:self.metadata_fuel_limit,
            wasm_bytes:&wasm_bytecode
        }).map_err(ProcessorError::MetadataError)?;

        let validated_metadata=
            validator::validator::validate_primitive(metadata)
            .map_err(ProcessorError::MetadataError)?;
        // 3 — introspect the compiled module for its declared ports/params.
        // A module that compiles but whose metadata is missing/malformed
        // must still fail the ticket — the DB definition must never drift
        // from the binary.
       

        let metadata_payload = match serde_json::to_string(&validated_metadata) {
            Ok(payload) => payload,
            Err(e) => {
                let message = format!("failed to serialize compiled metadata: {e}");
                self.mark_failed(event.ticket_id, message.clone()).await;
                return Err(ProcessorError::MetadataError(message));
            }
        };
        let name = validated_metadata.name;
        let description = validated_metadata.description;

        // 4 — store the artifact as a resource (bucket 1: compile check).
        // This never touches live state — a compile ticket is purely a check;
        // becoming the published transform is a separate, explicit action.
        let resource = self
            .data_provider
            .create_resource(
                event.ticket_id,
                wasm_bytecode,
                name,
                description,
                metadata_payload,
            )
            .await
            .map_err(|e| ProcessorError::DataError(e.to_string()))?;

        let ticket = self
            .data_provider
            .update_ticket(UpdateTicketParams {
                ticket_id: event.ticket_id,
                status: TicketStatus::Successful {
                    resource_id: resource.id,
                },
            })
            .await
            .map_err(|e| ProcessorError::DataError(e.to_string()))?;

        Ok(ProcessResult { ticket })
    }

    /// Best-effort: update ticket to Failed. Errors here are logged but not
    /// propagated so the original error is preserved for the caller.
    async fn mark_failed(
        &self,
        ticket_id: domain::db::ticket::db_ticket::TicketId,
        message: String,
    ) {
        if let Err(e) = self
            .data_provider
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
