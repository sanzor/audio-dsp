

use crate::audio_transform::{AudioTransform, AudioTransformFull, AudioTransformP};
use crate::filters::alpha;

#[derive(Debug)]
#[derive(Clone)]
pub struct AudioBuffer{
    pub samples:Vec<f32>,
    pub sample_rate:f32,
    pub channels:Channels
}
#[derive(Debug)]
#[derive(Clone)]
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
    
    fn low_pass(mut self,cutoff:f32)->Self {
        let mut prev_y=0.0;
        let alpha=alpha(cutoff, self.sample_rate);
        for sample in self.samples.iter_mut(){
            let y=alpha * (*sample)+ (1.0-alpha)*prev_y;
            *sample=y;
            prev_y=y;
        }
        return self;
    }

    fn high_pass(mut self,cutoff:f32)->Self {
        let mut prev_y=0.0;
        let mut prev_x:f32=0.0;
        let alpha=alpha(cutoff, self.sample_rate);
        for sample in self.samples.iter_mut(){
            let x=*sample;
            let y=alpha * (prev_y+x-prev_x);
            *sample=y;
            prev_y=y;
            prev_x=x;
        }
        return self;
    }
}

impl AudioTransformP for AudioBuffer{
    fn gain_p(mut self,gain:f32)->Self {
        let parallelism=AudioBuffer::thread_count();
        let _=std::thread::scope(|s|{
            let chunk_size = (self.samples.len() + parallelism - 1) / parallelism;
            let vec=self.samples.chunks_mut(chunk_size);
            for chunk in vec{
                let _=s.spawn(move||{
                    for val in chunk{
                        *val *=gain;
                    }
                });
            }
        });
        return self;
    }
    fn normalize_p(self)->AudioBuffer{
        let max_sample=self.max_sample();
        let normalized_buffer= self.gain_p(1.0/max_sample);
        normalized_buffer
    }
}

impl AudioBuffer{
    fn max_sample(&self)->f32{
        let max_sample=self.samples.iter().map(|el| el.abs()).fold(0.0, f32::max); 
        return max_sample
    }
    fn thread_count()->usize{
        std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
    }
}

impl AudioTransformFull for AudioBuffer{

}

