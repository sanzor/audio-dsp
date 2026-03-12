#[derive(Clone, Debug)]
pub struct InviteUserParams {
    pub email: String,
    pub project_id: String,
    pub role: String,
}
