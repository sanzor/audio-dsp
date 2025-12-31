use data_provider::region_provider::RegionsProvider;
use domain::regions::{
    add_region_params::AddRegionParams, copy_region_params::CopyRegionParams,
    edit_region_params::EditRegionParams,
};
use dtos::db::{region_db_dto::RegionDbDto, region_subtree::RegionSubtree};

pub struct DbRegionProvider {}

#[async_trait::async_trait]
impl RegionsProvider for DbRegionProvider {
    async fn get_region(&self, region_id: &str) -> Result<RegionDbDto, String> {
        let _ = region_id;
        todo!()
    }

    async fn get_regions_for_region_set(
        &self,
        region_set_id: &str,
    ) -> Result<Vec<RegionDbDto>, String> {
        let _ = region_set_id;
        todo!()
    }

    async fn add_region(&self, params: AddRegionParams) -> Result<RegionDbDto, String> {
        let _ = params;
        todo!()
    }

    async fn edit_region(&self, params: EditRegionParams) -> Result<RegionDbDto, String> {
        let _ = params;
        todo!()
    }

    async fn delete_region(&self, region_id: &str) -> Result<(), String> {
        let _ = region_id;
        todo!()
    }

    async fn copy_region(&self, params: CopyRegionParams) -> Result<RegionSubtree, String> {
        let _ = params;
        todo!()
    }

    async fn fetch_subtree(&self, region_id: &str) -> Result<RegionSubtree, String> {
        let _ = region_id;
        todo!()
    }
}
