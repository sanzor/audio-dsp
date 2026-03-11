use domain::actors::messages::player::get_player_state::GetPlayerStateResult;
use player::sink::AudioSink;

pub struct AttachSinkParams {
    pub user_id: String,
    pub track_id: String,
    pub sink: Box<dyn AudioSink + Send + Sync>,
}

pub struct AttachSinkResult {
    pub sink_id: String,
    pub track_id: String,
}

#[async_trait::async_trait]
pub trait PlayerProvider: Send + Sync {
    async fn play(&self, user_id: &str, track_id: &str) -> Result<(), String>;
    async fn pause(&self, user_id: &str, track_id: &str) -> Result<(), String>;
    async fn seek(&self, user_id: &str, track_id: &str, position: u32) -> Result<(), String>;
    async fn stop(&self, user_id: &str, track_id: &str) -> Result<(), String>;
    async fn attach_sink(&self, params: AttachSinkParams) -> Result<AttachSinkResult, String>;
    async fn remove_sink(&self, user_id: &str, track_id: &str, sink_id: &str) -> Result<(), String>;
    async fn get_state(&self, user_id: &str, track_id: &str) -> Result<GetPlayerStateResult, String>;
}
