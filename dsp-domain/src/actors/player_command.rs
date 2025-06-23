use serde::{Deserialize, Serialize};
#[derive(clap::Subcommand, Debug, Serialize, Deserialize)]
pub enum PlayerCommand {
    Play,
    Stop,
    Pause,
    Seek { position: u32 },
}
