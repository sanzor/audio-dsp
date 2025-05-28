use core::panic;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    time::Duration,
};

use audiolib::Channels;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use super::AudioSink;

pub struct CpalSink {
    device: cpal::Device,
    host: cpal::Host,
    config: cpal::StreamConfig,
    buffer: Arc<Mutex<VecDeque<f32>>>,
}
impl CpalSink {
    pub fn channels(&self) -> Channels {
        match self.config.channels {
            1 => Channels::Mono,
            2 => Channels::Stereo,
            n => panic!("Invalid sample rate"),
        }
    }
    pub(crate) fn sample_rate(&self) -> u16 {
        self.sample_rate()
    }
    pub fn new() -> Result<CpalSink, String> {
        let buffer = Arc::new(Mutex::new(VecDeque::new()));
        let buffer_clone = Arc::clone(&buffer);
        let host = cpal::default_host();
        let dev = host
            .default_output_device()
            .ok_or_else(|| "e".to_string())?;
        let config = dev
            .default_output_config()
            .map_err(|e| "e".to_string())?
            .config();
        let stream = dev
            .build_output_stream(
                &config,
                move |data: &mut [f32], _| {
                    let mut buf = buffer_clone.lock().unwrap();
                    for sample in data.iter_mut() {
                        *sample = buf.pop_front().unwrap_or(0.0);
                    }
                },
                |e| {},
                Some(Duration::from_micros(1000)),
            )
            .and_then(|st| {
                let _ = st.play();
                Ok(CpalSink {
                    config: config,
                    host: host,
                    device: dev,
                    buffer: buffer,
                })
            })
            .map_err(|e| format!("Could not build stream due to : {:?}", e));

        stream
    }
}

impl AudioSink for CpalSink {
    fn write_frame(&mut self, frame: &crate::AudioFrame) -> Result<(), String> {
        let mut buf = self.buffer.lock().map_err(|e| e.to_string())?;
        buf.extend(frame);
        Ok(())
    }
}
