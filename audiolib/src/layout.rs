use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
pub enum Channels {
    Mono = 1,
    Stereo = 2,
}

impl TryFrom<u8> for Channels {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Channels::Mono),
            2 => Ok(Channels::Stereo),
            _ => Err(format!("Invalid channel count {}", value)),
        }
    }
}

impl From<Channels> for u8 {
    fn from(value: Channels) -> Self {
        value as u8
    }
}
