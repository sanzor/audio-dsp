use async_trait::async_trait;

use crate::player_command::PlayerMessage;
#[async_trait]
pub trait AudioPlayerRef: Send + Sync + 'static {
    fn id(&self) -> String;
    async fn send_message(&self, message: PlayerMessage) -> Result<(), String>;
}
