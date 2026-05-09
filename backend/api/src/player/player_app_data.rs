use std::sync::Arc;

use super::player_provider::PlayerProvider;

#[derive(Clone)]
pub struct PlayerAppData {
    pub player_service: Arc<dyn PlayerProvider>,
}