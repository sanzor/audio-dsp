use std::collections::HashMap;

use data_provider::region_provider::RegionsProvider;
use domain::regions::{
    add_region_params::AddRegionParams, copy_region_params::CopyRegionParams,
    delete_region_params::DeleteRegionParams, edit_region_params::EditRegionParams, region::Region,
};
use dtos::db::{region_db_dto::RegionDbDto, region_subtree::RegionSubtree};
use tokio::sync::Mutex;
use ulid::Ulid;

use crate::db_providers::in_memory::in_memory_region_set_provider::InMemoryRegionSetProvider;

pub struct InMemoryRegionProvider {
    pub region_sets: Mutex<HashMap<String, RegionDbDto>>,
}

impl InMemoryRegionSetProvider {
    pub fn new() -> InMemoryRegionSetProvider {
        InMemoryRegionSetProvider {
            region_sets: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryRegionProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl RegionsProvider for InMemoryRegionProvider {
    async fn fetch_subtree(&self, region_id: &str) -> Result<RegionSubtree, String> {
        todo!()
    }

    async fn get_region(&self, region_id: &str) -> Result<RegionDbDto, String> {
        todo!()
    }

    /// Get all regions belonging to a region set (flat DTOs, no nested graphs)
    async fn get_regions_for_region_set(
        &self,
        region_set_id: &str,
    ) -> Result<Vec<RegionDbDto>, String>;
    // Child entity CRUD
    async fn add_region(&self, params: AddRegionParams) -> Result<RegionDbDto, String> {
        let mut guard = self.region_sets.lock().await;
        let set = match guard.get_mut(&params.region_set_id) {
            Some(set) => set,
            None => {
                return Err(format!(
                    "Could not find set with id {:?}",
                    params.region_set_id
                ))
            }
        };
        let region_id = Ulid::new();
        let region = Region {
            name: params.name,
            region_id: region_id.to_string(),
            region_set_id: params.region_set_id,
            end_time: params.end_time,
            start_time: params.start_time,
            graph: None,
        };
        set.regions.push(region);
        Ok(set.clone())
    }

    async fn edit_region(&self, params: EditRegionParams) -> Result<RegionDbDto, String> {
        let mut guard = self.region_sets.lock().await;
        let set = match guard.get_mut(&params.region_set_id) {
            Some(set) => set,
            None => {
                return Err(format!(
                    "Could not find set with id {:?}",
                    params.region_set_id
                ))
            }
        };
        let region_opt = set
            .regions
            .iter_mut()
            .find(|r| r.region_id == params.region_id)
            .ok_or_else(|| {
                format!("Could not find region with id {:?}", params.region_id).to_string()
            })?;
        if let Some(n) = params.name {
            region_opt.name = n;
        }
        if let Some(start_time) = params.start_time {
            region_opt.start_time = start_time;
        }
        if let Some(end_time) = params.end_time {
            region_opt.end_time = end_time;
        }
        Ok(set.clone())
    }

    async fn delete_region(&self, params: DeleteRegionParams) -> Result<(), String> {
        let mut guard = self.region_sets.lock().await;
        let set = match guard.get_mut(&params.region_set_id) {
            Some(set) => set,
            None => {
                return Err(format!(
                    "Could not find set with id {:?}",
                    params.region_set_id,
                ))
            }
        };
        let index = set
            .regions
            .iter()
            .position(|e| e.region_id == params.region_id)
            .ok_or_else(|| format!("Could not find region with id {:?}", params.region_id))?;

        set.regions.remove(index);
        Ok(())
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
