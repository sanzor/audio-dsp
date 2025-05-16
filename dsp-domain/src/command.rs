use std::fmt::Display;

use serde::Serialize;
pub enum DspCommand {
    Load {
        name: Option<String>,
        filename: Option<String>,
    },
    Upload {
        name: Option<String>,
        filename: Option<String>,
    },
    Delete {
        name: Option<String>,
    },
    Ls,
    Info {
        name: Option<String>,
    },
    Copy {
        name: Option<String>,
        copy_name: Option<String>,
    },
    Gain {
        name: Option<String>,
        gain: f32,
        mode: Option<RunMode>,
    },
    LowPass {
        name: Option<String>,
        cutoff: f32,
    },
    HighPass {
        name: Option<String>,
        cutoff: f32,
    },
    Normalize {
        name: Option<String>,
        mode: Option<RunMode>,
    },
    Exit,
    Play{
        name:Option<String>
    }
}

#[derive(Debug, Serialize)]
pub struct CommandResult {
    pub output: String,
}
impl Display for CommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", 1, 2)
    }
}
pub enum RunMode {
    Simple,
    Parallel { parallelism: u8 },
}
