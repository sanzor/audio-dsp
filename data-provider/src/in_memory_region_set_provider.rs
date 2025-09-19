use std::collections::HashMap;

use domain::regions::{
    add_region_params::AddRegionParams, create_region_set_params::CreateRegionSetParams,
    delete_region_params::DeleteRegionParams, edit_region_params::EditRegionParams,
    edit_region_set_params::EditRegionSetParams, region::Region, region_set::RegionSet,
};

use tokio::sync::Mutex;
use ulid::Ulid;

use crate::region_set_provider::RegionSetsProvider;

pub struct InMemoryRegionSetProvider {
    pub region_sets: Mutex<HashMap<String, RegionSet>>,
}

impl InMemoryRegionSetProvider {
    pub fn new() -> InMemoryRegionSetProvider {
        InMemoryRegionSetProvider {
            region_sets: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl RegionSetsProvider for InMemoryRegionSetProvider {
    async fn create_region_set(&self, params: CreateRegionSetParams) -> Result<RegionSet, String> {
        let region_set_id = Ulid::new();
        let region_set = RegionSet {
            track_length: params.track_length,
            track_id: params.track_id,
            regions: Vec::new(),
            name: params.name.unwrap_or(Ulid::new().to_string()),
            region_set_id: region_set_id.to_string(),
        };
        let mut guard = self.region_sets.lock().await;
        match guard.insert(region_set_id.to_string(), region_set.clone()) {
            Some(_) => Err("Could not insert the region set".to_string()),
            None => Ok(region_set),
        }
    }

    async fn get_region_set(&self, set_id: &str) -> Result<RegionSet, String> {
        let guard = self.region_sets.lock().await;
        let set = match guard.get(set_id) {
            Some(set) => Ok(set.clone()),
            None => Err(format!("Could not find set with id {:?}", set_id)),
        };
        return set;
    }

    async fn get_region_sets_for_track(&self, track_id: &str) -> Result<Vec<RegionSet>, String> {
        let guard = self.region_sets.lock().await;
        let sets = guard
            .values()
            .filter(|e| e.track_id == track_id)
            .cloned()
            .collect();

        return Ok(sets);
    }

   async fn get_region_sets(&self) -> Result<HashMap<String, Vec<RegionSet>>, String> {
        let guard = self.region_sets.lock().await;

        let mut rez: HashMap<String, Vec<RegionSet>> = HashMap::new();

        for elem in guard.values() {
        rez.entry(elem.track_id.clone()) // clone the String here
            .or_insert_with(Vec::new)
            .push(elem.clone());
        }

        Ok(rez)
}

    async fn edit_region_set(&self, params: EditRegionSetParams) -> Result<RegionSet, String> {
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
        if let Some(n) = params.name {
            set.name = n;
        }
        Ok(set.clone())
    }

    async fn delete_region_set(&self, set_id: &str) -> Result<(), String> {
        let mut guard = self.region_sets.lock().await;
        guard
            .remove(set_id)
            .ok_or_else(|| format!("Could not find set with id {:?}", set_id))
            .map(|_: RegionSet| ())
    }

    // Child entity CRUD
    async fn add_region(&self, params: AddRegionParams) -> Result<RegionSet, String> {
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
        };
        set.regions.push(region);
        Ok(set.clone())
    }

    async fn edit_region(&self, params: EditRegionParams) -> Result<RegionSet, String> {
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
                    params.region_set_id
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
}
