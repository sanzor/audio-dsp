use crate::domain::db::db_role_permission::DbRolePermission;

#[derive(Clone, Debug)]
pub struct CreateRolePermissionResult {
    pub role_permission: DbRolePermission,
}
