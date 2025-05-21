use crate::layout::Channels;
use hound::{WavReader, WavWriter};
use std::path::Path;

use crate::audio_buffer::AudioBuffer;
pub fn read_wav_file(filename: &Path) -> Result<AudioBuffer, String> {
    let mut reader = WavReader::open(filename).map_err(|e| e.to_string())?;
    let spec = reader.spec();
    let samples: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Int => reader
            .samples::<i32>()
            .map(|s| s.unwrap() as f32 / i16::MAX as f32)
            .collect(),
        hound::SampleFormat::Float => reader.samples().map(|s| s.unwrap()).collect(),
    };
    Ok(AudioBuffer {
        sample_rate: spec.sample_rate as f32,
        samples: samples,
        channels: match spec.channels {
            1 => Channels::Mono,
            2 => Channels::Stereo,
            _ => return Err("Unsupported number of channels".into()),
        },
    })
}

pub fn write_wav_file(audio_buffer: &AudioBuffer, filename: &Path) -> Result<(), String> {
    let wav_spec = hound::WavSpec {
        channels: match audio_buffer.channels {
            Channels::Mono => 1,
            Channels::Stereo => 2,
        },
        sample_rate: audio_buffer.sample_rate as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = WavWriter::create(filename, wav_spec)
        .map_err(|e| format!("Failed to create WAV writer: {}", e))?;

    for sample in &audio_buffer.samples {
        let scaled = (sample * i16::MAX as f32).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
        let _ = writer.write_sample(scaled);
    }
    let _ = writer
        .finalize()
        .map_err(|e| format!("Failed to finalize WAV {}", e));
    Ok(())
}
