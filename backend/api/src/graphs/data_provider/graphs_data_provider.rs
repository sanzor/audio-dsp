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
        save_graph_state_params::SaveGraphStateParams,
    },
};

#[async_trait::async_trait]
pub trait GraphsDataProvider: Send + Sync {
    async fn create_graph(&self, params: AddGraphParams) -> Result<DbGraph, String>;
    async fn copy_graph(&self, params: CopyGraphParams) -> Result<DbGraph, String>;
    async fn edit_graph(&self, params: EditGraphParams) -> Result<DbGraph, String>;
    async fn save_graph_state(&self, params: SaveGraphStateParams) -> Result<DbGraph, String>;
    async fn delete_graph(&self, params: DeleteGraphParams) -> Result<(), String>;
    async fn get_graph(&self, graph_id: &GraphId) -> Result<DbGraph, String>;
    async fn get_graph_for_region(
        &self,
        region_id: &RegionId,
    ) -> Result<Option<DbGraph>, String>;
}
