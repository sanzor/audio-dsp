use crate::roles::DbRole;

#[derive(Clone, Debug)]
pub struct GetRoleResult {
    pub role: DbRole,
}
