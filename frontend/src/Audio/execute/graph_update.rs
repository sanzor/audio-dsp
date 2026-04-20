use async_trait::async_trait;
use crate::types::active_graph::ActiveGraph;

#[async_trait]
pub trait GraphUpdate {
    async fn update_graph(&self, graph: ActiveGraph) -> Result<(), String>;
}
