use std::sync::Arc;

use crate::tracks::multipart_audio_parser::multipart_audio_parser_service::MultipartAudioParserService;

use super::tracks_provider::SourcesProvider;

#[derive(Clone)]
pub struct SourcesAppData {
    pub tracks_service: Arc<dyn SourcesProvider>,
    pub multipart_parser: Arc<MultipartAudioParserService>
}
