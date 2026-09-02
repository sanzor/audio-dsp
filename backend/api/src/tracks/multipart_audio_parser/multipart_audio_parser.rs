use actix_multipart::Multipart;
use domain::raw_track::RawTrack;

#[async_trait::async_trait(?Send)]
pub trait MultipartAudioParser: Send + Sync {
    async fn try_parse_multipart(&self, payload: Multipart) -> Result<RawTrack, String>;
}
