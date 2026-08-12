use domain::workspace_role::WorkspaceRole;

#[derive(Clone, Debug)]
pub struct InviteUserParams {
    pub email: String,
    pub workspace_id: i32,
    pub role: WorkspaceRole,
}
