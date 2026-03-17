use std::fmt;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "text", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ProjectRole {
    /// Global admin — never stored in DB, injected by the JWT middleware.
    SuperAdmin,
    Owner,
    Editor,
    Viewer,
}

impl ProjectRole {
    pub fn can_view(&self) -> bool {
        true // middleware already rejected non-members
    }

    pub fn can_edit(&self) -> bool {
        matches!(self, ProjectRole::SuperAdmin | ProjectRole::Owner | ProjectRole::Editor)
    }

    pub fn is_owner(&self) -> bool {
        matches!(self, ProjectRole::SuperAdmin | ProjectRole::Owner)
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "owner" => Some(ProjectRole::Owner),
            "editor" => Some(ProjectRole::Editor),
            "viewer" => Some(ProjectRole::Viewer),
            _ => None,
        }
    }
}

impl fmt::Display for ProjectRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectRole::SuperAdmin => write!(f, "superadmin"),
            ProjectRole::Owner => write!(f, "owner"),
            ProjectRole::Editor => write!(f, "editor"),
            ProjectRole::Viewer => write!(f, "viewer"),
        }
    }
}
