use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DspCommandResult {
    pub output: String,
    pub should_exit: bool,
}
