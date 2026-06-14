use std::{collections::HashSet, sync::Arc};

use domain::{
    db::db_transform::{DbTransform, DbTransformBinary, DbTransformDefinition, DbTransformPort, TransformId},
};
use wasmtime::*;
use crate::transforms::{compile_params::RequestCompileParams, compile_result::CompileResult, request_compile_result::RequestCompileResult, ticket::{ResourceId, TicketId, TicketStatusResult}};

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

    async fn request_compile_transform(&self,params:RequestCompileParams)->Result<RequestCompileResult,String>{
       
    }

    async fn get_compile_ticket_status(&self,id:TicketId)->Result<TicketStatusResult,String>{
        todo!()
    }
    async fn get_ticket_result(&self,id:ResourceId)->Result<CompileResult,String>{
        todo!()
    }

   
    async fn list_transform_summaries(&self, offset: i64, limit: i64) -> Result<(Vec<DbTransform>, i64), String> {
        self.data.list_transform_summaries(offset, limit).await
    }

    async fn get_transform_definition(&self, id: TransformId) -> Result<DbTransformDefinition, String> {
        self.data.get_transform_definition(id).await
    }

    async fn get_transform_definitions(&self, ids: &[TransformId]) -> Result<Vec<DbTransformDefinition>, String> {
        let definitions = self.data.get_transform_definitions(ids).await?;
        let missing_ids = Self::collect_missing_ids(ids, &definitions, |definition| definition.transform_id);
        if missing_ids.is_empty() {
            Ok(definitions)
        } else {
            Err(format!("Transforms not found: {:?}", missing_ids))
        }
    }

    async fn get_transform_binary(&self, id: TransformId) -> Result<Vec<u8>, String> {
        self.storage.get_transform_binary(id).await
    }

    async fn get_transform_binaries(&self, ids: &[TransformId]) -> Result<Vec<DbTransformBinary>, String> {
        let binaries = self.storage.get_transform_binaries(ids).await?;
        let missing_ids = Self::collect_missing_ids(ids, &binaries, |binary| binary.transform_id);
        if missing_ids.is_empty() {
            Ok(binaries)
        } else {
            Err(format!("Transform binaries not found: {:?}", missing_ids))
        }
    }

    async fn create_transform(
        &self,
        name: String,
        description: Option<String>,
        icon: Option<String>,
    ) -> Result<DbTransformDefinition, String> {
        let db = self.data.insert_transform(name, description, icon).await?;
        self.data.get_transform_definition(db.transform_id).await
    }

    async fn update_transform(
        &self,
        id: TransformId,
        name: String,
        description: Option<String>,
        icon: Option<String>,
    ) -> Result<DbTransformDefinition, String> {
        let db = self.data.update_transform(id, name, description, icon).await?;
        self.data.get_transform_definition(db.transform_id).await
    }

    async fn delete_transform(&self, id: TransformId) -> Result<(), String> {
        self.data.delete_transform(id).await
    }

    async fn add_port(
        &self,
        transform_id: TransformId,
        name: String,
        direction: String,
        port_order: i32,
        description: Option<String>,
    ) -> Result<DbTransformPort, String> {
        self.data.insert_port(transform_id, name, direction, port_order, description).await
    }

    async fn delete_port(&self, port_id: i64) -> Result<(), String> {
        self.data.delete_port(port_id).await
    }
}
