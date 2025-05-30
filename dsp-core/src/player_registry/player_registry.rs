use async_trait::async_trait;
use player::player_ref::AudioPlayerRef;
#[async_trait]
pub trait AudioPlayerRegistry: Send + Sync {
    async fn upsert(&self, id: &str, player: Box<dyn AudioPlayerRef>) -> Result<(), String>;
    async fn get_by_id(&self, id: &str) -> Result<Box<dyn AudioPlayerRef>, String>;
    async fn get_all_ids(&self) -> Vec<String>;
    async fn remove(&self, id: &str) -> Result<Box<dyn AudioPlayerRef>, String>;
}
