use domain::project_role::ProjectRole;

#[derive(Clone, Debug)]
pub struct AcceptInviteResult {
    pub user_id: i64,
    pub project_id: i64,
    pub role: ProjectRole,
}
