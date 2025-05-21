pub trait CommandChannel<C>: Send + Sync {
    fn send(&self, command: C) -> Result<(), String>;
}
