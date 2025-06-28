use crate::actors::user_update_params::UserUpdateParams;

pub enum UserCommand {
    Remove,
    Update(UserUpdateParams),
}
