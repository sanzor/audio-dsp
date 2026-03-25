use domain::project_role::ProjectRole;

#[derive(Clone, Debug)]
pub struct AcceptInviteResult {
    pub user_id: i32,
    pub project_id: i32,
    pub role: ProjectRole,
}
