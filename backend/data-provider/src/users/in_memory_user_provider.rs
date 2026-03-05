use std::{collections::HashMap, future::Future, pin::Pin, sync::Mutex};

use domain::{create_domain_user_params::CreateDomainUserParams, domain_user::DomainUser};
use ulid::Ulid;

use super::user_provider::UserProvider;

pub struct InMemoryUserProvider {
    users: Mutex<HashMap<String, DomainUser>>,
}

impl InMemoryUserProvider {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryUserProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl UserProvider for InMemoryUserProvider {
    fn get_user_by_id<'a>(
        &'a self,
        id: &'a str,
    ) -> Pin<Box<dyn Future<Output = Option<DomainUser>> + Send + 'a>> {
        let result = self
            .users
            .lock()
            .unwrap()
            .get(id)
            .cloned();
        Box::pin(async move { result })
    }

    fn get_user_by_google_sub_id<'a>(
        &'a self,
        sub_id: &'a str,
    ) -> Pin<Box<dyn Future<Output = Option<DomainUser>> + Send + 'a>> {
        let result = self
            .users
            .lock()
            .unwrap()
            .values()
            .find(|u| u.google_sub_id.as_deref() == Some(sub_id))
            .cloned();
        Box::pin(async move { result })
    }

    fn create_domain_user<'a>(
        &'a self,
        user_params: CreateDomainUserParams,
    ) -> Pin<Box<dyn Future<Output = Result<DomainUser, String>> + Send + 'a>> {
        let id = Ulid::new().to_string();
        let user = DomainUser {
            id: id.clone(),
            email: user_params.email,
            google_sub_id: user_params.google_sub_id,
        };
        self.users.lock().unwrap().insert(id, user.clone());
        Box::pin(async move { Ok(user) })
    }

    fn update_user<'a>(
        &self,
        user: DomainUser,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>> {
        self.users.lock().unwrap().insert(user.id.clone(), user);
        Box::pin(async move { Ok(()) })
    }

    fn delete_user<'a>(
        &'a self,
        id: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<DomainUser, String>> + Send + 'a>> {
        let result = self
            .users
            .lock()
            .unwrap()
            .remove(id)
            .ok_or_else(|| format!("User {} not found", id));
        Box::pin(async move { result })
    }

    fn list_users<'a>(&'a self) -> Pin<Box<dyn Future<Output = Vec<DomainUser>> + Send + 'a>> {
        let users: Vec<DomainUser> = self.users.lock().unwrap().values().cloned().collect();
        Box::pin(async move { users })
    }
}
