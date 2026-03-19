use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: i64,
    pub name: Option<String>,
    pub email: Option<String>,
    pub is_admin: bool,
    pub project_id: Option<i64>,
    pub exp: usize,
    /// "access" | "refresh" | "verification" | "invite"
    pub purpose: Option<String>,
    /// role embedded in invite and project-scoped tokens
    pub role: Option<String>,
}
