use std::io::Cursor;

use crate::{audio_buffer::AudioBuffer, decoded_audio::DecodedAudio, Channels};

pub fn encode_audio_buffer_as_wav(buffer:&AudioBuffer)->Result<Vec<u8>,String>{
        let spec=hound::WavSpec{
            channels:match buffer.channels{
                Channels::Mono=>1,
                Channels::Stereo=>2
            },
            sample_rate:buffer.sample_rate as u32,
            bits_per_sample:16,
            sample_format:hound::SampleFormat::Int
        };
        let mut cursor=Cursor::new(Vec::new());
        let mut writer=hound::WavWriter::new(&mut cursor,spec).map_err(|e|format!("Failed to create wav writer {}",e))?;
        for sample in buffer.samples.iter(){
            let clamped=sample.clamp(-1.0, 1.0);
            let i16_sample=(clamped*i16::MAX as f32) as i16;
            writer.write_sample(i16_sample)
            .map_err(|e| format!("Could not write sample :{}",e))?;
        }
        writer.finalize().map_err(|e|format!("Finalize error: {}",e))?;
        Ok(cursor.into_inner())
}

pub fn decode_canonical_audio(data:&[u8])->Result<DecodedAudio,String>{
    let mut reader=hound::WavReader::new(std::io::Cursor::new(data)).map_err(|e| e.to_string())?;
    let spec=reader.spec();
    let samples:Vec<f32>=reader
        .samples::<i16>()
        .map(|s|s.unwrap() as f32 /i16::MAX as f32)
        .collect();
    let channels= match spec.channels{
         1 => Channels::Mono,
          2=>Channels::Stereo,
        _ => return Err("Invalid channels".into())}
          ;
    Ok(DecodedAudio { samples: samples, sample_rate: spec.sample_rate, channels: Channels::from(channels) })
}


    