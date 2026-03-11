use crate::permissions::DbPermission;

#[derive(Clone, Debug)]
pub struct UpdatePermissionResult {
    pub permission: DbPermission,
}
