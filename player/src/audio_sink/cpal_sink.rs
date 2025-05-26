use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait};

use super::AudioSink;

pub struct CpalSink{
    device:cpal::Device,
    host:cpal::Host,
    config:cpal::SupportedStreamConfig
}
impl CpalSink{
    pub fn new()->Result<CpalSink,String>{
        let host=cpal::default_host();
        let dev=host.default_output_device().ok_or_else(||"e".to_string())?;
        let config=dev.default_output_config().map_err(|e|"e".to_string())?.config();
        dev.build_output_stream(&config, |data,info|{
            
        }, |e|{}, Some(Duration::from_micros(1000)));
        Ok(CpalSink{host:host,device:dev,config:config})
    }
}
impl AudioSink for CpalSink{
    fn write_frame(&mut self, frame: crate::AudioFrame) -> Result<(), String> {
        
    }
}