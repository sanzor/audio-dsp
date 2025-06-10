use serde::Serialize;

#[derive(Debug, Serialize)]

pub struct TracksMessageResult {
    pub output: String,
    pub should_exit: bool,
}
