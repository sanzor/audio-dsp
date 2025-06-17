use kameo::Reply;
use serde::Serialize;
#[derive(Reply)]
#[derive(Debug, Serialize)]
pub struct TracksMessageResult {
    pub output: String,
    pub should_exit: bool,
}
