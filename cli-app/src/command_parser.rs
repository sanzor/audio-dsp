use dsp_domain::{
    audio_player_message::AudioPlayerMessage,
    dsp_message::{DspMessage, RunMode},
};
pub(crate) struct CommandParser {}

impl CommandParser {
    pub(crate) fn parse_crud_command(&self, input: &str) -> Result<DspMessage, String> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() < 1 {
            return Err("No command provided".to_string());
        }
        let command = parts.get(0).map(|v| v.to_ascii_lowercase());
        let payload = &parts[1..];
        let rez = match command.as_deref() {
            Some("load") => self.parse_load(payload),
            Some("save") => self.parse_upload(payload),
            Some("info") => self.parse_info(payload),
            Some("unload") => self.parse_unload(payload),
            Some("list") => self.parse_ls(payload),
            Some("gain") => self.parse_gain(payload),
            Some("normalize") => self.parse_normalize(payload),
            Some("low_pass") => self.parse_low_pass(payload),
            Some("high_pass") => self.parse_high_pass(payload),
            Some("run-script") => self.parse_run_script(payload),
            _ => Err("Could not process comand".to_string()),
        };
        rez
    }

    pub(crate) fn parse_player_command(&self, input: &str) -> Result<AudioPlayerMessage, String> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() < 1 {
            return Err("No command provided".to_string());
        }
        let command = parts.get(0).map(|v| v.to_ascii_lowercase());
        let payload = &parts[1..];
        let rez = match command.as_deref() {
            Some("play") => self.parse_play(payload),
            Some("pause") => self.parse_pause(payload),
            Some("stop") => self.parse_stop(payload),
            _ => Err("Could not process comand".to_string()),
        };
        rez
    }
    fn parse_load(&self, value: &[&str]) -> Result<DspMessage, String> {
        match value {
            [user_name, filename, track_name] => Ok(DspMessage::Load {
                user_name: Some(user_name.to_string()),
                filename: Some(filename.to_string()),
                track_name: Some(track_name.to_string()),
            }),
            [user_name, filename] => Ok(DspMessage::Load {
                user_name: Some(user_name.to_string()),
                filename: Some(filename.to_string()),
                track_name: Some(filename.to_string()),
            }),
            _ => Err("Invalid load command".to_owned()),
        }
    }
    fn parse_upload(&self, value: &[&str]) -> Result<DspMessage, String> {
        match value {
            [user_name, filename, track_name] => Ok(DspMessage::Upload {
                user_name: Some(user_name.to_string()),
                filename: Some(filename.to_string()),
                track_name: Some(track_name.to_string()),
            }),
            [user_name, track_name] => Ok(DspMessage::Upload {
                user_name: Some(user_name.to_string()),
                filename: Some(track_name.to_string()),
                track_name: Some(track_name.to_string()),
            }),
            _ => Err("Invalid save command".to_owned()),
        }
    }

    fn parse_unload(&self, value: &[&str]) -> Result<DspMessage, String> {
        match value {
            [user_name, track_name] => Ok(DspMessage::Delete {
                user_name: Some(user_name.to_string()),
                track_name: Some(track_name.to_string()),
            }),
            _ => Err("Invalid unload command".to_owned()),
        }
    }

    fn parse_info(&self, value: &[&str]) -> Result<DspMessage, String> {
        match value {
            [user_name, track_name] => Ok(DspMessage::Info {
                user_name: Some(user_name.to_string()),
                track_name: Some(track_name.to_string()),
            }),
            _ => Err("Invalid info command".to_owned()),
        }
    }

    fn parse_ls(&self, value: &[&str]) -> Result<DspMessage, String> {
        match value {
            [user_name] => Ok(DspMessage::Ls {
                user_name: Some(user_name.to_string()),
            }),
            _ => Err("".to_string()),
        }
    }

    fn parse_gain(&self, value: &[&str]) -> Result<DspMessage, String> {
        match value {
            [user_name, track_name, factor] => factor
                .parse::<f32>()
                .map(|f| DspMessage::Gain {
                    user_name: Some(user_name.to_string()),
                    track_name: Some(track_name.to_string()),
                    gain: f,
                    mode: Some(RunMode::Simple),
                    parallelism: None,
                })
                .map_err(|e| e.to_string()),
            _ => Err("Invalid gain command".to_owned()),
        }
    }
    fn parse_normalize(&self, value: &[&str]) -> Result<DspMessage, String> {
        match value {
            [user_name, track_name] => Ok(DspMessage::Normalize {
                user_name: Some(user_name.to_string()),
                track_name: Some(track_name.to_string()),
                mode: Some(RunMode::Simple),
                parallelism: None,
            }),
            _ => Err("Invalid load command".to_owned()),
        }
    }
    fn parse_low_pass(&self, value: &[&str]) -> Result<DspMessage, String> {
        match value {
            [user_name, cutoff, track_name] => cutoff
                .parse::<f32>()
                .map(|f| DspMessage::LowPass {
                    user_name: Some(user_name.to_string()),
                    track_name: Some(track_name.to_string()),
                    cutoff: f,
                })
                .map_err(|e| e.to_string()),
            _ => Err("Invalid low_pass command".to_owned()),
        }
    }
    fn parse_high_pass(&self, value: &[&str]) -> Result<DspMessage, String> {
        match value {
            [user_name, cutoff, name] => cutoff
                .parse::<f32>()
                .map(|f| DspMessage::HighPass {
                    user_name: Some(user_name.to_string()),
                    track_name: Some(name.to_string()),
                    cutoff: f,
                })
                .map_err(|e| e.to_string()),
            _ => Err("Invalid high_pass command".to_owned()),
        }
    }

    fn parse_play(&self, value: &[&str]) -> Result<AudioPlayerMessage, String> {
        match value {
            [track_name] => Ok(AudioPlayerMessage::Play {
                track_id: Some(track_name.to_string()),
            }),
            _ => Err("Invalid run command".to_string()),
        }
    }

    fn parse_pause(&self, value: &[&str]) -> Result<AudioPlayerMessage, String> {
        match value {
            [user_name, track_name] => Ok(AudioPlayerMessage::Pause {
                track_id: Some(track_name.to_string()),
            }),
            _ => Err("Invalid run command".to_string()),
        }
    }
    fn parse_stop(&self, value: &[&str]) -> Result<AudioPlayerMessage, String> {
        match value {
            [user_name, track_name] => Ok(AudioPlayerMessage::Stop {
                track_id: Some(track_name.to_string()),
            }),
            _ => Err("Invalid run command".to_string()),
        }
    }

    fn parse_run_script(&self, value: &[&str]) -> Result<DspMessage, String> {
        match value {
            [user_id, file] => Ok(DspMessage::RunScript {
                user_name: Some(user_id.to_string()),
                file: file.to_string(),
            }),
            _ => Err("Invalid run command".to_string()),
        }
    }
}
