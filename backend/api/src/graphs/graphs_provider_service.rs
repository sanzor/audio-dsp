use std::sync::Arc;

use domain::{
    db::{
        db_graph::{DbGraph, GraphId},
        db_region::RegionId,
    },
    graphs::{
        add_graph_params::AddGraphParams,
        copy_graph_params::CopyGraphParams,
        delete_graph_params::DeleteGraphParams,
        edit_graph_params::EditGraphParams,
    },
};

use crate::domain::service_error::ServiceError;

use super::{
    data_provider::graphs_data_provider::GraphsDataProvider, graphs_provider::GraphsProvider,
};

pub struct GraphsProviderService {
    data: Arc<dyn GraphsDataProvider>,
}

impl GraphsProviderService {
    pub fn new(data: Arc<dyn GraphsDataProvider>) -> Self {
        Self { data }
    }
}

#[async_trait::async_trait]
impl GraphsProvider for GraphsProviderService {
    async fn create_graph(&self, params: AddGraphParams) -> Result<DbGraph, ServiceError> {
        self.data.create_graph(params).await.map_err(ServiceError::from)
    }

    async fn copy_graph(&self, params: CopyGraphParams) -> Result<DbGraph, ServiceError> {
        self.data.copy_graph(params).await.map_err(ServiceError::from)
    }

    async fn edit_graph(&self, params: EditGraphParams) -> Result<DbGraph, ServiceError> {
        self.data.edit_graph(params).await.map_err(ServiceError::from)
    }

    async fn delete_graph(&self, params: DeleteGraphParams) -> Result<(), ServiceError> {
        self.data.delete_graph(params).await.map_err(ServiceError::from)
    }

    async fn get_graph(&self, graph_id: &GraphId) -> Result<DbGraph, ServiceError> {
        self.data.get_graph(graph_id).await.map_err(ServiceError::from)
    }

    async fn get_graph_for_region(
        &self,
        region_id: &RegionId,
    ) -> Result<Option<DbGraph>, ServiceError> {
        self.data
            .get_graph_for_region(region_id)
            .await
            .map_err(ServiceError::from)
    }
}
