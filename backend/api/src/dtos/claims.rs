use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub is_admin: bool,
    pub workspace_id: Option<i32>,
    pub exp: usize,
    /// "access" | "refresh" | "verification" | "invite"
    pub purpose: Option<String>,
    /// role embedded in invite and workspace-scoped tokens
    pub role: Option<String>,
}
