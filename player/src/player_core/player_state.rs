#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum PlayerState {
    Stopped,
    Paused,
    Playing,
}
