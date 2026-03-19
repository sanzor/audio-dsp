pub trait User {
    fn id(&self) -> i64;
    fn name(&self) -> &str;
    fn email(&self) -> &str;
}
