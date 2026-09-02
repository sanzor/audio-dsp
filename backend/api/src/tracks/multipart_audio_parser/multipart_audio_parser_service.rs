use audiolib::{audio_buffer::AudioBuffer, utils::decode_canonical_audio, Channels};
use domain::raw_track::{RawTrack, TrackInfo};
use futures_util::StreamExt;
use std::str::FromStr;
use tracing::{error, info, warn};

pub struct MultipartAudioParserService {}

impl MultipartAudioParserService {
    pub async fn try_parse_multipart(
        &self,
        p: actix_multipart::Multipart,
    ) -> Result<domain::raw_track::RawTrack, String> {
        let mut name: Option<String> = None;
        let mut payload = p;
        let mut extension: Option<String> = None;
        let mut sample_rate: Option<f32> = None;
        let mut channels: Option<Channels> = None;
        let mut samples_bytes: Vec<u8> = vec![];

        while let Some(Ok(mut field)) = payload.next().await {
            let field_name = field.name().unwrap_or("").to_string();
            let field_data = Self::next_multipart_field(&mut field).await;

            match field_name.as_str() {
                "name" => name = Some(String::from_utf8_lossy(&field_data).to_string()),
                "extension" => extension = Some(String::from_utf8_lossy(&field_data).to_string()),
                "sample_rate" => {
                    sample_rate = String::from_utf8_lossy(&field_data).parse::<f32>().ok()
                }
                "channels" => {
                    channels = Channels::from_str(&String::from_utf8_lossy(&field_data)).ok()
                }
                "samples" => samples_bytes = field_data,
                _ => {}
            }
        }
        let (name, extension) = match (name, extension) {
            (Some(n), Some(ext)) => (n, ext),
            _ => {
                warn!("add-track-multi rejected: missing required fields");
                return Err("Missing required fields".into());
            }
        };

        if samples_bytes.is_empty() {
            warn!("add-track-multi rejected: missing samples data");
            return Err("Missing samples data".into());
        }

        let audio_buffer = if self.is_wav_bytes(&samples_bytes) {
            match decode_canonical_audio(&samples_bytes) {
                Ok(decoded) => AudioBuffer {
                    samples: decoded.samples,
                    sample_rate: decoded.sample_rate as f32,
                    channels: decoded.channels,
                },
                Err(err) => {
                    error!(error = %err, "add-track-multi rejected: invalid wav payload");
                    return Err("Invalid wav payload".into());
                }
            }
        } else {
            let (sample_rate, channels) = match (sample_rate, channels) {
                (Some(sr), Some(ch)) => (sr, ch),
                _ => {
                    warn!("add-track-multi rejected: missing raw audio metadata");
                    return Err("Missing required fields".into());
                }
            };
            AudioBuffer {
                samples: Self::bytes_to_f32(samples_bytes),
                sample_rate,
                channels,
            }
        };

        let length = audio_buffer.samples.len() as f32
            / (audio_buffer.sample_rate * Self::channel_count(audio_buffer.channels) as f32);
        info!(
            track_name = %name,
            extension = %extension,
            sample_rate = audio_buffer.sample_rate,
            sample_count = audio_buffer.samples.len(),
            "add-track-multi payload parsed"
        );
        let raw_track = RawTrack {
            info: TrackInfo {
                name,
                extension,
                length,
            },
            data: audio_buffer,
        };
        Ok(raw_track)
    }

    async fn next_multipart_field(field: &mut actix_multipart::Field) -> Vec<u8> {
        let mut data = Vec::new();
        while let Some(chunk) = field.next().await {
            data.extend_from_slice(&chunk.unwrap());
        }
        data
    }

    fn is_wav_bytes(&self, bytes: &[u8]) -> bool {
        bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WAVE"
    }

    fn bytes_to_f32(bytes: Vec<u8>) -> Vec<f32> {
        use std::convert::TryInto;
        let mut out = Vec::with_capacity(bytes.len() / 4);
        for chunk in bytes.chunks_exact(4) {
            let arr: [u8; 4] = chunk.try_into().unwrap();
            out.push(f32::from_le_bytes(arr));
        }
        out
    }

    fn channel_count(channels: Channels) -> usize {
        match channels {
            Channels::Mono => 1,
            Channels::Stereo => 2,
        }
    }
}
