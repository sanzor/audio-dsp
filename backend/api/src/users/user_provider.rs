use std::{future::Future, pin::Pin};

use domain::{create_domain_user_params::CreateDomainUserParams, domain_user::DomainUser};

#[async_trait::async_trait]
pub trait UserProvider: Send + Sync {
    fn get_user_by_id<'a>(
        &'a self,
        id: i64,
    ) -> Pin<Box<dyn Future<Output = Option<DomainUser>> + Send + 'a>>;
    fn get_user_by_google_sub_id<'a>(
        &'a self,
        sub_id: &'a str,
    ) -> Pin<Box<dyn Future<Output = Option<DomainUser>> + Send + 'a>>;
    fn get_user_by_email<'a>(
        &'a self,
        email: &'a str,
    ) -> Pin<Box<dyn Future<Output = Option<DomainUser>> + Send + 'a>>;
    fn create_domain_user<'a>(
        &'a self,
        user_params: CreateDomainUserParams,
    ) -> Pin<Box<dyn Future<Output = Result<DomainUser, String>> + Send + 'a>>;
    fn update_user<'a>(
        &self,
        user: DomainUser,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>>;
    fn delete_user<'a>(
        &'a self,
        id: i64,
    ) -> Pin<Box<dyn Future<Output = Result<DomainUser, String>> + Send + 'a>>;
    fn list_users<'a>(&'a self) -> Pin<Box<dyn Future<Output = Vec<DomainUser>> + Send + 'a>>;
}
