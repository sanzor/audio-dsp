use std::sync::Arc;

use data_provider::{tracks_provider::TracksProvider, user_provider::UserProvider};

use crate::user_actor::player_factory::PlayerFactory;

pub struct UserActorDeps {
    pub user_provider: Arc<dyn UserProvider>,
    pub player_factory: Arc<PlayerFactory>,
    pub tracks_provider: Arc<dyn TracksProvider + Send + Sync + 'static>,
}
