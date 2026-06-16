use crate::domain_user::UserId;

pub trait User {
    fn id(&self) -> UserId;
    fn name(&self) -> &str;
    fn email(&self) -> &str;
}
