use domain::project_role::ProjectRole;

#[derive(Clone, Debug)]
pub struct InviteUserParams {
    pub email: String,
    pub project_id: String,
    pub role: ProjectRole,
}
