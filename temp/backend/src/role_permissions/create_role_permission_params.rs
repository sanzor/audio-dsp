#[derive(Clone, Debug)]
pub struct CreateRolePermissionParams {
    pub role_id: crate::domain::db::db_role::RoleId,
    pub permission_id: crate::domain::db::db_permission::PermissionId,
}
