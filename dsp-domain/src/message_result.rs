use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MessageResult {
    pub output: String,
    pub should_exit: bool,
}
