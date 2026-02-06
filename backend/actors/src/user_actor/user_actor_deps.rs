use std::sync::Arc;

use data_provider::{region_set_provider::RegionSetsProvider, tracks_provider::TracksProvider};

use crate::user_actor::player_factory::PlayerFactory;

pub struct UserActorDeps {
    pub player_factory: Arc<PlayerFactory>,
    pub tracks_provider: Arc<dyn TracksProvider + Send + Sync>,
    pub region_sets_provider: Arc<dyn RegionSetsProvider + Send + Sync>,
}
