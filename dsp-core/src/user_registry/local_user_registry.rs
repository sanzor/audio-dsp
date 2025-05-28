use std::{collections::HashMap, sync::Mutex};

use dsp_domain::domain_user::DomainUser;

use super::user_registry::UserRegistry;

pub struct LocalUserRegistry{
    users:Mutex<HashMap<String,DomainUser>>
}
impl UserRegistry for LocalUserRegistry{
    fn upsert(&self,id:String,player:dsp_domain::domain_user::DomainUser)->Result<(),String> {
        todo!()
    }

    fn get_user_by_id(&self,id:String)->Option<dsp_domain::domain_user::DomainUser> {
        todo!()
    }

    fn get_all_ids(&self)->Vec<String> {
        todo!()
    }

    fn remove(&self,id:String)->Option<dsp_domain::domain_user::DomainUser> {
        todo!()
    }
}
impl LocalUserRegistry{
    pub fn new()->LocalUserRegistry{
        LocalUserRegistry { users: Mutex::new(HashMap::new()) }
    }
}