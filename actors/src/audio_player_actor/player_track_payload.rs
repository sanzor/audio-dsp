use audiolib::decoded_audio::DecodedAudio;
use domain::track_meta::TrackMeta;

pub struct PlayerTrackPayload{
    pub audio:DecodedAudio,
    pub meta:TrackMeta
}