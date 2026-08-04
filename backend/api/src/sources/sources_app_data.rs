use std::sync::Arc;

use crate::sources::multipart_audio_parser::multipart_audio_parser_service::SourceMultipartAudioParserService;

use super::sources_provider::SourcesProvider;

#[derive(Clone)]
pub struct SourcesAppData {
    pub sources_service: Arc<dyn SourcesProvider>,
    pub multipart_parser: Arc<SourceMultipartAudioParserService>,
}
