use crate::audio_transform::AudioTransform;
#[derive(Debug)]
pub struct AudioBuffer{
    pub samples:Vec<f32>,
    pub sample_rate:f32,
    pub channels:Channels
}
#[derive(Debug)]
pub enum Channels{
    Mono=1,
    Stereo=2
}

impl AudioTransform for AudioBuffer{
    fn gain(&mut self,factor:f32)->&mut AudioBuffer{
        let _ = self.samples.iter_mut().for_each(|el| *el *=factor);
        return self;
    }
}

