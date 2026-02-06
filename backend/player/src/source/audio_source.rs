use std::{future::Future, pin::Pin};

use crate::AudioFrame;

pub trait AudioSource {
    fn next_frame<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Option<AudioFrame>> + Send + 'a>>;
}
