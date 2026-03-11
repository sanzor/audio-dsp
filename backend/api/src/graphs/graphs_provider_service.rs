use std::sync::Arc;

use domain::{
    db::{
        db_graph::{DbGraph, GraphId},
        db_region::RegionId,
    },
    graphs::{
        add_graph_params::AddGraphParams,
        delete_graph_params::DeleteGraphParams,
        edit_graph_params::EditGraphParams,
        edge::Edge,
        graph_subtree::GraphSubtree,
        node::Node,
    },
};

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
    async fn create_graph(&self, params: AddGraphParams) -> Result<DbGraph, String> {
        self.data.create_graph(params).await
    }

    async fn edit_graph(&self, params: EditGraphParams) -> Result<DbGraph, String> {
        self.data.edit_graph(params).await
    }

    async fn delete_graph(&self, params: DeleteGraphParams) -> Result<(), String> {
        self.data.delete_graph(params).await
    }

    async fn get_graph(&self, graph_id: &GraphId) -> Result<DbGraph, String> {
        self.data.get_graph(graph_id).await
    }

    async fn get_graph_for_region(&self, region_id: &RegionId) -> Result<Option<DbGraph>, String> {
        self.data.get_graph_for_region(region_id).await
    }

    async fn get_graph_subtree(&self, graph_id: &GraphId) -> Result<GraphSubtree, String> {
        let graph = self.data.get_graph(graph_id).await?;
        let db_nodes = self.data.get_nodes(graph_id).await?;
        let db_edges = self.data.get_edges(graph_id).await?;

        Ok(GraphSubtree {
            graph_id: graph.graph_id,
            region_id: graph.region_id,
            name: graph.name,
            nodes: db_nodes
                .into_iter()
                .map(|n| Node {
                    id: n.node_id,
                    graph_id: n.graph_id,
                })
                .collect(),
            edges: db_edges
                .into_iter()
                .map(|e| Edge {
                    id: e.edge_id,
                    graph_id: e.graph_id,
                })
                .collect(),
        })
    }
}
