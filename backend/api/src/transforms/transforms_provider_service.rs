use std::{collections::HashSet, sync::Arc};

use domain::db::{db_transform::{DbTransform, DbTransformBinary, DbTransformDefinition, DbTransformPort, TransformId}, ticket::{create_ticket_params::CreateTicketParams, db_resource::ResourceId, db_ticket::{DbTicket, TicketId}}};
use wasmtime::*;
use crate::{domain::{ service_error::ServiceError},
         transforms::{compile_params::RequestCompileParams, compile_result::CompileResult}};

use super::{
    data_provider::transforms_data_provider::TransformsDataProvider,
    storage_provider::transform_storage_provider::TransformStorageProvider,
    transforms_provider::TransformsProvider,
};

pub struct TransformsProviderService {
    data: Arc<dyn TransformsDataProvider>,
    storage: Arc<dyn TransformStorageProvider>,

}

impl TransformsProviderService {
    pub fn new(
        data: Arc<dyn TransformsDataProvider>,
        storage: Arc<dyn TransformStorageProvider>,
    ) -> Self {
        Self { data, storage }
    }

    fn collect_missing_ids<T, F>(requested_ids: &[TransformId], items: &[T], get_id: F) -> Vec<TransformId>
    where
        F: Fn(&T) -> TransformId,
    {
        let found: HashSet<TransformId> = items.iter().map(get_id).collect();
        requested_ids
            .iter()
            .copied()
            .filter(|id| !found.contains(id))
            .collect()
    }
}

#[async_trait::async_trait]
impl TransformsProvider for TransformsProviderService {

    async fn request_compile_transform(&self,params:RequestCompileParams)->Result<DbTicket,ServiceError>{
        self.data.create_ticket(CreateTicketParams{
            transform_id:params.transform_id,
            user_id:params.user_id,
            source_code:params.payload
       }).await.map_err(ServiceError::from)

    }

    async fn get_compile_ticket_status(&self,id:TicketId)->Result<DbTicket,ServiceError>{
        self.data.get_ticket(id)
            .await
            .map_err(ServiceError::from)
    }
    async fn get_ticket_result(&self,id:ResourceId)->Result<CompileResult,ServiceError>{
        todo!()
    }

   
    async fn list_transform_summaries(&self, offset: i64, limit: i64) -> Result<(Vec<DbTransform>, i64), ServiceError> {
        self.data.list_transform_summaries(offset, limit).await.map_err(ServiceError::from)
    }

    async fn get_transform_definition(&self, id: TransformId) -> Result<DbTransformDefinition, ServiceError> {
        self.data.get_transform_definition(id).await.map_err(ServiceError::from)
    }

    async fn get_transform_definitions(&self, ids: &[TransformId]) -> Result<Vec<DbTransformDefinition>, ServiceError> {
        let definitions = self.data.get_transform_definitions(ids).await?;
        let missing_ids = Self::collect_missing_ids(ids, &definitions, |definition| definition.transform_id);
        if missing_ids.is_empty() {
            Ok(definitions)
        } else {
            // ServiceError::NotFound(format!("Transforms not found: {:?}", missing_ids))
            Err(ServiceError::NotFound)
        }
    }

    async fn get_transform_binary(&self, id: TransformId) -> Result<Vec<u8>, ServiceError> {
        self.storage.get_transform_binary(id).await.map_err(ServiceError::from)
    }

    async fn get_transform_binaries(&self, ids: &[TransformId]) -> Result<Vec<DbTransformBinary>, ServiceError> {
        let binaries = self.storage.get_transform_binaries(ids).await?;
        let missing_ids = Self::collect_missing_ids(ids, &binaries, |binary| binary.transform_id);
        if missing_ids.is_empty() {
            Ok(binaries)
        } else {
             Err(ServiceError::NotFound)
        }
    }

    async fn create_transform(
        &self,
        name: String,
        description: Option<String>,
        icon: Option<String>,
    ) -> Result<DbTransformDefinition, ServiceError> {
        let db = self.data.insert_transform(name, description, icon).await?;
        self.data.get_transform_definition(db.transform_id).await.map_err(ServiceError::from)
    }

    async fn update_transform(
        &self,
        id: TransformId,
        name: String,
        description: Option<String>,
        icon: Option<String>,
    ) -> Result<DbTransformDefinition, ServiceError> {
        let db = self.data.update_transform(id, name, description, icon).await?;
        self.data.get_transform_definition(db.transform_id).await.map_err(ServiceError::from)
    }

    async fn delete_transform(&self, id: TransformId) -> Result<(), ServiceError> {
        let v=self.data.delete_transform(id).await.map_err(ServiceError::from);
        v
    }

    async fn add_port(
        &self,
        transform_id: TransformId,
        name: String,
        direction: String,
        port_order: i32,
        description: Option<String>,
    ) -> Result<DbTransformPort, ServiceError> {
        self.data.insert_port(transform_id, name, direction, port_order, description).await.map_err(ServiceError::from)
    }

    async fn delete_port(&self, port_id: i64) -> Result<(), ServiceError> {
        self.data.delete_port(port_id).await.map_err(ServiceError::from)
    }
}
