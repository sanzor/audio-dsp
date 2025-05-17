use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DspCommandResult {
    pub output: String,
}
