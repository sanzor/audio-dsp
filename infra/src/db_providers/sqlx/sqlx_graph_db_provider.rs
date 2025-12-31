use data_provider::graph_provider::GraphsProvider;
use domain::graphs::{add_graph_params::AddGraphParams, edit_graph_params::EditGraphParams};
use dtos::db::{graph_db_dto::GraphDbDto, graph_subtree::GraphSubtree};

pub struct GraphDbProvider;

#[async_trait::async_trait]
impl GraphsProvider for GraphDbProvider {
    async fn get_graph(&self, graph_id: &str) -> Result<GraphDbDto, String> {
        let _ = (self, graph_id);
        todo!()
    }

    async fn get_graphs_for_region(&self, region_id: &str) -> Result<Vec<GraphDbDto>, String> {
        let _ = (self, region_id);
        todo!()
    }

    async fn add_graph(&self, params: AddGraphParams) -> Result<GraphDbDto, String> {
        let _ = (self, params);
        todo!()
    }

    async fn edit_graph(&self, params: EditGraphParams) -> Result<GraphDbDto, String> {
        let _ = (self, params);
        todo!()
    }

    async fn copy_graph(
        &self,
        source_graph_id: &str,
        destination_region_id: &str,
        graph_copy_name: &str,
    ) -> Result<GraphSubtree, String> {
        let _ = (
            self,
            source_graph_id,
            destination_region_id,
            graph_copy_name,
        );
        todo!()
    }

    async fn fetch_subtree(&self, graph_id: &str) -> Result<GraphSubtree, String> {
        let _ = (self, graph_id);
        todo!()
    }

    async fn delete_graph(&self, graph_id: &str) -> Result<(), String> {
        let _ = (self, graph_id);
        todo!()
    }
}
