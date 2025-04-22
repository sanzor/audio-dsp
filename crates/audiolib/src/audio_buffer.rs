use crate::audio_transform::AudioTransform;
use crate::filters::{self, alpha};

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
    fn gain(mut self,factor:f32)->AudioBuffer{
        if self.max_sample()>0.0{
            self.samples.iter_mut().for_each(|sample| *sample *=factor);
        }
        return self
    }

    fn normalize(self)->AudioBuffer {
        let max_sample=self.max_sample();
        let normalized_buffer= self.gain(1.0/max_sample);
        normalized_buffer
    }
    
    fn low_pass(self,cutoff:f32)->Self {
        let mut result=Vec::with_capacity(self.samples.len());
        let mut prev_y=0.0;
        let alpha=alpha(cutoff, self.sample_rate);
        for sample in &self.samples{
            let y=alpha * (*sample)+ (1.0-alpha)*prev_y;
            result.push(y);
            prev_y=y;
        }
        return self;
    }

    fn high_pass(self,cutoff:f32)->Self {
        let mut result=Vec::with_capacity(self.samples.len());
        let mut prev_y=0.0;
        let mut prev_x:f32=0.0;
        let alpha=alpha(cutoff, self.sample_rate);
        for sample in &self.samples{
            let y=alpha * (prev_y+*sample-prev_x);
            result.push(y);
            prev_y=y;
            prev_x=*sample;
        }
        return self;
    }
}
impl AudioBuffer{
    fn max_sample(&self)->f32{
        let max_sample=self.samples.iter().map(|el| el.abs()).fold(0.0, f32::max); 
        return max_sample
    }
}

