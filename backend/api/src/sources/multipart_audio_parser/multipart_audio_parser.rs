use actix_multipart::Multipart;
use domain::sources::raw_source::RawSource;

#[async_trait::async_trait(?Send)]
pub trait MultipartAudioParser: Send + Sync {
    async fn try_parse_multipart(&self, payload: Multipart) -> Result<RawSource, String>;
}
