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
        graph_subtree::GraphSubtree,
    },
};

#[async_trait::async_trait]
pub trait GraphsProvider: Send + Sync {
    async fn create_graph(&self, params: AddGraphParams) -> Result<DbGraph, String>;
    async fn edit_graph(&self, params: EditGraphParams) -> Result<DbGraph, String>;
    async fn delete_graph(&self, params: DeleteGraphParams) -> Result<(), String>;
    async fn get_graph(&self, graph_id: &GraphId) -> Result<DbGraph, String>;
    async fn get_graph_for_region(&self, region_id: &RegionId) -> Result<Option<DbGraph>, String>;
    async fn get_nodes(&self, graph_id: &GraphId) -> Result<Vec<DbNode>, String>;
    async fn get_edges(&self, graph_id: &GraphId) -> Result<Vec<DbEdge>, String>;
    /// Returns the full graph with nodes and edges — used when the graph canvas opens.
    async fn get_graph_subtree(&self, graph_id: &GraphId) -> Result<GraphSubtree, String>;
}
