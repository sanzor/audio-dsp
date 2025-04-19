use crate::audio_transform::AudioTransform;

pub struct AudioBuffer{
    pub samples:Vec<f32>,
    pub sample_rate:f32,
    pub channels:Channels
}

pub enum Channels{
    Mono=1,
    Stereo=2
}

impl AudioTransform for AudioBuffer{
    fn gain(mut self,factor:f32)->AudioBuffer{
        for sample in &mut self.samples{
            *sample *=factor;
        }
        self
    }
}