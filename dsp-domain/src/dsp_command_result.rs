use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DspCommandResult {
    pub output: String,
}

impl Display for DspCommandResult{
     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.output)
    }
}
