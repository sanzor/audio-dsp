use hound::WavReader;

use crate::{audio_buffer::{AudioBuffer, Channels}, audio_transform::AudioTransform};
pub fn parse_wav(filename:&str)->Result<impl AudioTransform,hound::Error>{
    let reader=WavReader::open(filename)?;
    let samples:Vec<f32>=reader.into_samples::<i16>()
                                .map(|elem| elem.unwrap() as f32/i16::MAX as f32)
                                .collect();
    return Ok(AudioBuffer{sample_rate:33_f32,samples:samples,channels:Channels::Mono});

}




