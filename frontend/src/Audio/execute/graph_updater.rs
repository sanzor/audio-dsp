// The update path — separate from the play path.
//
// GraphUpdater recompiles whenever the graph changes and swaps the new compiled
// state into the shared GraphExecutor.  The play path (Mixer) holds an Arc to
// the same executor and picks up the new state on the next block without
// locking or blocking the audio loop beyond one Mutex acquisition.

use std::sync::Arc;
use async_trait::async_trait;
use crate::types::active_graph::ActiveGraph;
use crate::types::transform_store::TransformStore;
use crate::graph;
use super::graph_executor::GraphExecutor;
use super::graph_update::GraphUpdate;
use super::wasm_instance::WasmInstance;




pub struct GraphUpdater {
    executor: Arc<GraphExecutor>,
    transform_store: TransformStore,
    load_instances: Box<dyn Fn(Vec<u32>) -> Vec<Box<dyn WasmInstance>> + Send + Sync>,
}

impl GraphUpdater {
    pub fn new(
        executor: Arc<GraphExecutor>,
        transform_store: TransformStore,
        // Caller provides a function that loads WASM instances for a list of transform IDs.
        // This keeps the executor free of any WASM loading logic.
        load_instances: Box<dyn Fn(Vec<u32>) -> Vec<Box<dyn WasmInstance>> + Send + Sync>,
    ) -> Self {
        Self { executor, transform_store, load_instances }
    }
}

#[async_trait]
impl GraphUpdate for GraphUpdater {
    async fn update_graph(&self, graph: ActiveGraph) -> Result<(), String> {
        let compile_output = graph::compile_graph(&graph, &self.transform_store)?;
        let instances = (self.load_instances)(compile_output.transform_ids.clone());

        self.executor.load(compile_output, instances, 128);
        Ok(())
    }
}
