use crate::roles::DbRole;

#[derive(Clone, Debug)]
pub struct UpdateRoleResult {
    pub role: DbRole,
}
