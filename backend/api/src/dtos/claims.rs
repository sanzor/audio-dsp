use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub is_admin: bool,
    pub project_id: Option<String>,
    pub exp: usize,
    /// "access" | "refresh" | "verification" | "invite"
    pub purpose: Option<String>,
    /// role embedded in invite tokens
    pub invited_role: Option<String>,
}
