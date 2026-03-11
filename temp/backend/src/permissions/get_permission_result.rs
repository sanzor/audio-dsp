use crate::services::permissions::permission::Permission;

#[derive(Clone, Debug)]
pub struct GetPermissionResult {
    pub permission: Permission,
}
