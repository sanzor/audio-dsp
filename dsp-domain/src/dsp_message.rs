use std::fmt::Display;

use actix::Message;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use crate::dsp_message_result::DspMessageResult;
#[derive(clap::Subcommand, Debug, Serialize, Deserialize, Message)]
#[rtype(result = "Result<DspMessageResult, String>")]
pub enum DspMessage {
    Load {
        user_name: Option<String>,
        track_name: Option<String>,
        filename: Option<String>,
    },
    Upload {
        user_name: Option<String>,
        track_name: Option<String>,
        filename: Option<String>,
    },
    Delete {
        user_name: Option<String>,
        track_name: Option<String>,
    },
    Ls {
        user_name: Option<String>,
    },
    Info {
        user_name: Option<String>,
        track_name: Option<String>,
    },
    Copy {
        user_name: Option<String>,
        track_name: Option<String>,
        copy_name: Option<String>,
    },

    Exit {
        user_name: Option<String>,
    },
    Play {
        user_id: Option<String>,
        track_id: Option<String>,
    },
    RunScript {
        user_name: Option<String>,
        #[arg(help = "Path to batch JSON file")]
        file: String,
    },
    Gain {
        user_name: Option<String>,
        track_name: Option<String>,
        gain: f32,
        mode: Option<RunMode>,
        #[arg(long)]
        parallelism: Option<u8>,
    },
    LowPass {
        user_name: Option<String>,
        track_name: Option<String>,
        cutoff: f32,
    },
    HighPass {
        user_name: Option<String>,
        track_name: Option<String>,
        cutoff: f32,
    },
    Normalize {
        user_name: Option<String>,
        track_name: Option<String>,
        mode: Option<RunMode>,
        #[arg(long)]
        parallelism: Option<u8>,
    },
    Stop {
        user_id: Option<String>,
        track_id: Option<String>,
    },
    Pause {
        user_id: Option<String>,
        track_id: Option<String>,
    },
}

impl Display for DspMessageResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", 1, 2)
    }
}
#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
pub enum RunMode {
    Simple,
    Parallel,
}
