use domain::project_role::ProjectRole;

#[derive(Clone, Debug)]
pub struct AcceptInviteResult {
    pub user_id: String,
    pub project_id: String,
    pub role: ProjectRole,
    /// project-scoped JWT
    pub token: String,
}
