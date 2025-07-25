use crate::{create_domain_user_params::CreateDomainUserParams, domain_user::DomainUser};



pub enum UserActorInitInput{
    Existing(DomainUser),
    New(CreateDomainUserParams)
}