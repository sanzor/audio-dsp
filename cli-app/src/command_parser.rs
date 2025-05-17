use dsp_domain::dsp_command::{DspCommand, RunMode};
pub(crate) struct CommandParser {}

impl CommandParser {
    pub (crate) fn parse_command(&self,input: &str) -> Result<DspCommand, String> {
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
            Some("play")=>self.parse_play(payload),
            Some("run-script")=>self.parse_run_script(payload),
            _ => Err("Could not process comand".to_string()),
        };
        rez
    }
    fn parse_load(&self,value: &[&str]) -> Result<DspCommand, String> {
        match value {
            [filename, name] => Ok(DspCommand::Load {
                filename: Some(filename.to_string()),
                name: Some(name.to_string()),
            }),
            [filename] => Ok(DspCommand::Load {
                filename: Some(filename.to_string()),
                name: Some(filename.to_string()),
            }),
            _ => Err("Invalid load command".to_owned()),
        }
    }
    fn parse_upload(&self,value: &[&str]) -> Result<DspCommand, String> {
        match value {
            [filename, name] => Ok(DspCommand::Upload {
                filename: Some(filename.to_string()),
                name: Some(name.to_string()),
            }),
            [name] => Ok(DspCommand::Upload {
                filename: Some(name.to_string()),
                name: Some(name.to_string()),
            }),
            _ => Err("Invalid save command".to_owned()),
        }
    }

    fn parse_unload(&self,value: &[&str]) -> Result<DspCommand, String> {
        match value {
            [name] => Ok(DspCommand::Delete {
                name: Some(name.to_string()),
            }),
            _ => Err("Invalid unload command".to_owned()),
        }
    }

    fn parse_info(&self,value: &[&str]) -> Result<DspCommand, String> {
        match value {
            [name] => Ok(DspCommand::Info {
                name: Some(name.to_string()),
            }),
            _ => Err("Invalid info command".to_owned()),
        }
    }

    fn parse_ls(&self,value: &[&str]) -> Result<DspCommand, String> {
        let _ = value;
        Ok(DspCommand::Ls)
    }

    fn parse_gain(&self,value: &[&str]) -> Result<DspCommand, String> {
        match value {
            [name, factor] => factor
                .parse::<f32>()
                .map(|f| DspCommand::Gain {
                    name: Some(name.to_string()),
                    gain: f,
                    mode: Some(RunMode::Simple),
                    parallelism: None,
                })
                .map_err(|e| e.to_string()),
            _ => Err("Invalid gain command".to_owned()),
        }
    }
    fn parse_normalize(&self,value: &[&str]) -> Result<DspCommand, String> {
        match value {
            [name] => Ok(DspCommand::Normalize {
                name: Some(name.to_string()),
                mode: Some(RunMode::Simple),
                parallelism: None,
            }),
            _ => Err("Invalid load command".to_owned()),
        }
    }
    fn parse_low_pass(&self,value: &[&str]) -> Result<DspCommand, String> {
        match value {
            [cutoff, name] => cutoff
                .parse::<f32>()
                .map(|f| DspCommand::LowPass {
                    name: Some(name.to_string()),
                    cutoff: f,
                })
                .map_err(|e| e.to_string()),
            _ => Err("Invalid low_pass command".to_owned()),
        }
    }
    fn parse_high_pass(&self,value: &[&str]) -> Result<DspCommand, String> {
        match value {
            [cutoff, name] => cutoff
                .parse::<f32>()
                .map(|f| DspCommand::HighPass {
                    name: Some(name.to_string()),
                    cutoff: f,
                })
                .map_err(|e| e.to_string()),
            _ => Err("Invalid high_pass command".to_owned()),
        }
    }

    fn parse_play(&self,value: &[&str]) -> Result<DspCommand, String> {
        match value {
            [name] => Ok(DspCommand::Play {
                name: Some(name.to_string()),
            }),
            _ => Err("Invalid run command".to_string()),
        }
    }

     fn parse_run_script(&self,value: &[&str]) -> Result<DspCommand, String> {
        match value {
            [file] => Ok(DspCommand::RunScript { file: file.to_string() }),
            _ => Err("Invalid run command".to_string()),
        }
    }
}
