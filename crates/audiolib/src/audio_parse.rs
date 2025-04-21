use hound::WavReader;

use crate::{audio_buffer::{AudioBuffer, Channels}, audio_transform::AudioTransform};
pub fn read_wav_file(filename:&str)->Result<impl AudioTransform,String>{
    let mut reader=WavReader::open(filename).map_err(|e| e.to_string())?;
    let spec=reader.spec();
    let samples:Vec<f32>=match spec.sample_format{
        hound::SampleFormat::Int=>reader.samples::<i32>()
                                  .map(|s|s.unwrap() as f32/i16::MAX as f32)
                                  .collect(),
        hound::SampleFormat::Float=>reader.samples()
                                    .map(|s|s.unwrap())
                                    .collect(),
    };
    Ok(AudioBuffer{
        sample_rate:spec.sample_rate as f32,
        samples:samples,
        channels:match spec.channels{
            1=>Channels::Mono,
            2=>Channels::Stereo,
            _=>return Err("Unsupported number of channels".into())
        }})
}




