use crate::AudioFrame;



#[async_trait::async_trait]
pub trait AudioSink {
    async fn write_frame(
        & mut self,
        frame: AudioFrame,
    ) -> Result<(), String>;
}
