    use dsp_domain::domain_user::DomainUser;

        pub trait UserRegistry:Send+Sync{
            fn upsert(&self,id:String,player:DomainUser)->Result<(),String>;
            fn get_user_by_id(&self,id:String)->Option<DomainUser>;
            fn get_all_ids(&self)->Vec<String>;
            fn remove(&self,id:String)->Option<DomainUser>;
        }