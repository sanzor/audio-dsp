use crate::roles::DbRole;

#[derive(Clone, Debug)]
pub struct CreateRoleResult {
    pub role: DbRole,
}
