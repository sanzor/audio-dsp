use domain::{
    db::{
        db_edge::DbEdge,
        db_graph::{DbGraph, GraphId},
        db_node::DbNode,
        db_region::RegionId,
    },
    graphs::{
        add_graph_params::AddGraphParams,
        delete_graph_params::DeleteGraphParams,
        edit_graph_params::EditGraphParams,
    },
};

#[async_trait::async_trait]
pub trait GraphsDataProvider: Send + Sync {
    async fn create_graph(&self, params: AddGraphParams) -> Result<DbGraph, String>;
    async fn edit_graph(&self, params: EditGraphParams) -> Result<DbGraph, String>;
    async fn delete_graph(&self, params: DeleteGraphParams) -> Result<(), String>;
    async fn get_graph(&self, graph_id: &GraphId) -> Result<DbGraph, String>;
    async fn get_graph_for_region(&self, region_id: &RegionId) -> Result<Option<DbGraph>, String>;
    async fn get_nodes(&self, graph_id: &GraphId) -> Result<Vec<DbNode>, String>;
    async fn get_edges(&self, graph_id: &GraphId) -> Result<Vec<DbEdge>, String>;
}
