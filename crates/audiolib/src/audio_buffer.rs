use std::thread;

use crate::audio_transform::AudioTransform;
use crate::filters::alpha;

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
impl AudioBuffer{
    fn max_sample(&self)->f32{
        let max_sample=self.samples.iter().map(|el| el.abs()).fold(0.0, f32::max); 
        return max_sample
    }
    fn gain_parallel(mut self,gain:f32)->Self{
       
        let parallelism=std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
        let len=self.samples.len();
        let chunk_size=(len+parallelism-1)/parallelism;
        let ptr=self.samples.as_mut_ptr();
        let mut handles=Vec::with_capacity(parallelism);

        for chunk_start in (0..len).step_by(chunk_size){
            let chunkend=usize::min(chunk_start+chunk_size, len);
            let chunk_len=chunkend-chunk_start;
            if chunk_len==0{
                continue
            }
           
            let handle = thread::spawn({
                let ptr = self.samples.as_mut_ptr(); // this is ok
                move || {
                    unsafe {
                        let chunk_ptr = ptr.add(chunk_s); // pointer stays inside closure
                        let chunk = std::slice::from_raw_parts_mut(chunk_ptr, chunk_len);
            
                        for sample in chunk {
                            *sample *= gain;
                        }
                    }
                }
            });
        }

        self
    }
}

