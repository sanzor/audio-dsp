use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use domain::{create_domain_user_params::CreateDomainUserParams, domain_user::DomainUser};
use tokio::sync::Mutex;
use ulid::Ulid;

use crate::user_provider::{UserProvider};

pub struct InMemoryUserProvider{
    pub users:Arc<Mutex<HashMap<String,DomainUser>>>
}

impl InMemoryUserProvider{
    pub fn new()->impl UserProvider{
        InMemoryUserProvider{users:Arc::new(Mutex::new(HashMap::new()))}
    }
}

impl UserProvider for InMemoryUserProvider{
    fn get_user_by_id<'a>(&'a self,id:&'a str)->Pin<Box<dyn Future<Output=Option<DomainUser>>+Send+'a>>{
        Box::pin(async move{
             let guard=self.users.lock().await;
             let rez=guard.get(id).cloned();
             rez
        })
       
    }

    fn get_user_by_google_sub_id<'a>(&'a self,id:&'a str)->Pin<Box<dyn Future<Output=Option<DomainUser>>+Send+'a>>{
        Box::pin(async move{
             let guard=self.users.lock().await;
             let rez=guard.values().find(|u| match &u.google_sub_id{
                Some(str)=>str==id,
                None=>false
             }).cloned();
             rez
        })
       
    }

    fn delete_user<'a>(&'a self,id: &'a str) -> Pin<Box<dyn Future<Output = Result<DomainUser,String> > +Send+'a> > {
        Box::pin(async move{
            let mut state=self.users.lock().await;
            match state.remove_entry(id){
                Some((k,deleted))=>Ok(deleted),
                None=>Err("could not find user".into())
            }
        })
    }
    fn list_users<'a>(&'a self)->Pin<Box<dyn Future<Output=Vec<DomainUser>>+Send+'a>>{
        Box::pin(async move{
            let users= self.users.lock().await;
            let iter=users.values().cloned().collect();
            iter
        })
       
    }
    fn update_user<'a>(&self, user: DomainUser) ->  Pin<Box<dyn Future<Output = Result<(), String>>+Send+'a>>{
        todo!()
     }
    fn create_domain_user<'a>(&'a self,user_params:CreateDomainUserParams)->Pin<Box<dyn Future<Output = Result<DomainUser,String>>+Send+'a>>{
        Box::pin(async move{
            let id=Ulid::new();
        let user=DomainUser{
            id:id.to_string(),
            google_sub_id:user_params.google_sub_id,
            email:user_params.email,
            name:user_params.name,
            picture:user_params.picture
        };
        let mut insert=self.users.lock().await;
        insert.insert(id.to_string(),user.clone());
        Ok(user)
        })
     }
     
}