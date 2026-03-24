use std::sync::Arc;

use crate::tracks::multipart_audio_parser::multipart_audio_parser_service::MultipartAudioParserService;

use super::tracks_provider::TracksProvider;

#[derive(Clone)]
pub struct TracksAppData {
    pub tracks_service: Arc<dyn TracksProvider>,
    pub multipart_parser: Arc<MultipartAudioParserService>
}
