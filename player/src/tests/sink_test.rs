use audiolib::{audio_parse, Channels};
use rstest::rstest;

use crate::audio_sink::{cpal_sink::CpalSink, AudioSink};
#[rstest]
#[ignore]
pub fn test_cpal_sink() -> Result<(), String> {
    let path = std::path::Path::new("../audio_data/dragons.wav");
    let mut sink = CpalSink::new()?;
    let data = audio_parse::read_wav_file(path)?;
    let wav_channels = data.channels as usize;
    let sink_channels = sink.channels() as usize;

    for chunk in data.samples.chunks_exact(wav_channels) {
        let frame = match (wav_channels, sink_channels) {
            (1, 1) => vec![chunk[0]],
            (1, 2) => vec![chunk[0], chunk[0]],
            (2, 2) => vec![chunk[0], chunk[1]],
            (2, 1) => vec![(chunk[0] + chunk[1]) * 0.5],
            _ => return Err("Invalid channels".to_string()),
        };
        let _ = sink.write_frame(&frame.to_vec());
    }
    Ok(())
}
