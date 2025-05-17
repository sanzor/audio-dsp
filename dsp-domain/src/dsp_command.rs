use std::fmt::Display;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use crate::dsp_command_result::DspCommandResult;
#[derive(clap::Subcommand, Debug, Serialize, Deserialize)]
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
        #[arg(long)]
        parallelism: Option<u8>,
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
        #[arg(long)]
        parallelism: Option<u8>,
    },
    Exit,
    Play {
        name: Option<String>,
    },
    RunScript {
        #[arg(help = "Path to batch JSON file")]
        file: String,
    },
}

impl Display for DspCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", 1, 2)
    }
}
#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
pub enum RunMode {
    Simple,
    Parallel,
}
