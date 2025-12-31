use domain::graphs::{add_graph_params::AddGraphParams, edit_graph_params::EditGraphParams};
use dtos::db::{graph_db_dto::GraphDbDto, graph_subtree::GraphSubtree};

#[async_trait::async_trait]
pub trait GraphsProvider: Send + Sync {
    /// Fetch a single graph (flat DTO).
    async fn get_graph(&self, graph_id: &str) -> Result<GraphDbDto, String>;

    /// List graphs for a region (each returned as a flat DTO).
    async fn get_graphs_for_region(&self, region_id: &str) -> Result<Vec<GraphDbDto>, String>;

    /// Create a new graph (returns the created flat DTO).
    async fn add_graph(&self, params: AddGraphParams) -> Result<GraphDbDto, String>;

    /// Update a graph (returns the updated flat DTO).
    async fn edit_graph(&self, params: EditGraphParams) -> Result<GraphDbDto, String>;

    /// Copy a graph (returns the copied subtree)
    async fn copy_graph(
        &self,
        source_graph_id: &str,
        destination_region_id: &str,
        graph_copy_name: &str,
    ) -> Result<GraphSubtree, String>;

    /// Fetch the full subtree rooted at the graph.
    async fn fetch_subtree(&self, graph_id: &str) -> Result<GraphSubtree, String>;

    /// Delete a graph by id.
    async fn delete_graph(&self, graph_id: &str) -> Result<(), String>;
}
