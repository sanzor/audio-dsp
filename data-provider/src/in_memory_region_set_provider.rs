use std::collections::HashMap;

use domain::regions::region_set::RegionSet;
use tokio::sync::Mutex;
use ulid::Ulid;

use crate::region_set_provider::RegionSetsProvider;

pub struct InMemoryRegionSetProvider{
    pub region_sets:Mutex<HashMap<String,RegionSet>>
}

impl InMemoryRegionSetProvider{
    pub fn new()->InMemoryRegionSetProvider{
        InMemoryRegionSetProvider { region_sets: HashMap::new() }
    }
}

#[async_trait::async_trait]
impl RegionSetsProvider for InMemoryRegionSetProvider{
     async fn create_region_set(&self, track_id: &str, name: Option<String>)
        -> Result<RegionSet, String>{

                let region_set_id=Ulid::new();
                let region_set=RegionSet{
                    regions:Vec::new(),
                    track_id:track_id.to_string(),
                    region_set_id:region_set_id.to_string()
                };
                let guard=self.region_sets.lock().await;
                match guard.insert(region_set_id.to_string(),region_set){
                    Some(_)=>Err("Could not insert the region set".to_string()),
                    None=>Ok()
                }
            
        }

    async fn get_region_set(&self, set_id: &str)
        -> Result<RegionSet, String>{

        }

    async fn update_region_set(&self, set_id: &str, name: Option<String>)
        -> Result<RegionSet, String>;

    async fn delete_region_set(&self, set_id: &str)
        -> Result<(), String>;

    // Child entity CRUD
    async fn add_region(&self, set_id: &str, params: AddRegionParams)
        -> Result<RegionSet, String>;

    async fn edit_region(&self, set_id: &str, params: EditRegionParams)
        -> Result<RegionSet, String>;

    async fn delete_region(&self, set_id: &str, region_id: &str)
        -> Result<RegionSet, String>;
}