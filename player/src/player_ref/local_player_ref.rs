use std::sync::mpsc::Sender;

use async_trait::async_trait;

use crate::player_command::PlayerMessage;

use super::player_ref::AudioPlayerRef;

pub struct LocalPlayerRef {
    pub id: String,
    pub tx: Sender<PlayerMessage>,
}
#[async_trait]
impl AudioPlayerRef for LocalPlayerRef {
    // async fn send_message(&self, message: PlayerMessage) -> Result<(), String> {
    //     self.tx
    //         .send(message)
    //         .map_err(|_| "Disconnected".to_string())
    // }

    async fn id(&self) -> String {
        self.id.clone()
    }

    async fn send_message(&self, message: PlayerMessage) -> Result<(), String> {
        // your logic
        Ok(())
    }
}
