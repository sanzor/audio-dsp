use domain::project_role::ProjectRole;

#[derive(Clone, Debug)]
pub struct InviteUserParams {
    pub email: String,
    pub project_id: i32,
    pub role: ProjectRole,
}
