use kameo::Reply;

#[derive(Reply)]
pub struct AudioPlayerMessageResult {
    pub output: String,
    pub should_exit: bool,
}
