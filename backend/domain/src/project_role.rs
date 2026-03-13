use std::fmt;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "text", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ProjectRole {
    Owner,
    Editor,
    Viewer,
}

impl fmt::Display for ProjectRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectRole::Owner => write!(f, "owner"),
            ProjectRole::Editor => write!(f, "editor"),
            ProjectRole::Viewer => write!(f, "viewer"),
        }
    }
}

impl ProjectRole {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "owner" => Some(ProjectRole::Owner),
            "editor" => Some(ProjectRole::Editor),
            "viewer" => Some(ProjectRole::Viewer),
            _ => None,
        }
    }
}
