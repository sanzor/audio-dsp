use kameo::Reply;
use serde::Serialize;
#[derive(Reply, Debug, Serialize)]
pub struct TracksMessageResult {
    pub output: String,
    pub should_exit: bool,
}
