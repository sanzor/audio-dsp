use std::sync::Arc;

use data_provider::{
    region_sets::region_sets_provider::RegionSetsProvider,
    tracks::tracks_provider::TracksProvider,
};

use crate::user_actor::player_factory::PlayerFactory;

pub struct UserActorDeps {
    pub player_factory: Arc<PlayerFactory>,
    pub tracks_provider: Arc<dyn TracksProvider + Send + Sync>,
    pub region_sets_provider: Arc<dyn RegionSetsProvider + Send + Sync>,
}
