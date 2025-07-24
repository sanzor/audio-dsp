pub trait User {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn email(&self) -> &str;
}
