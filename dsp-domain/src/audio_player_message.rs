use serde::{Deserialize, Serialize};
#[derive(clap::Subcommand, Debug, Serialize, Deserialize)]
pub enum AudioPlayerMessage {
    Play { track_id: Option<String> },
    Stop { track_id: Option<String> },
    Pause { track_id: Option<String> },
}
