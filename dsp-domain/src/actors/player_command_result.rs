use kameo::Reply;
use serde::{Deserialize, Serialize};

#[derive(Reply, Serialize, Deserialize)]
pub struct PlayerCommandResult {
    pub output: String,
    pub should_exit: bool,
}
