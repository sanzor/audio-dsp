use domain::workspace_role::WorkspaceRole;

#[derive(Clone, Debug)]
pub struct AcceptInviteResult {
    pub user_id: i32,
    pub workspace_id: i32,
    pub role: WorkspaceRole,
}
