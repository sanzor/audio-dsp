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

#[async_trait::async_trait]
pub trait GraphsProvider: Send + Sync {
    async fn create_graph(&self, params: AddGraphParams) -> Result<DbGraph, ServiceError>;
    async fn copy_graph(&self, params: CopyGraphParams) -> Result<DbGraph, ServiceError>;
    async fn edit_graph(&self, params: EditGraphParams) -> Result<DbGraph, ServiceError>;
    async fn delete_graph(&self, params: DeleteGraphParams) -> Result<(), ServiceError>;
    async fn get_graph(&self, graph_id: &GraphId) -> Result<DbGraph, ServiceError>;
    async fn get_graph_for_region(&self, region_id: &RegionId) -> Result<Option<DbGraph>, ServiceError>;
}
