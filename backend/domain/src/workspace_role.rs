use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "text", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum WorkspaceRole {
    /// Global admin — never stored in DB, injected by the JWT middleware.
    SuperAdmin,
    Owner,
    Editor,
    Viewer,
}

impl WorkspaceRole {
    pub fn can_view(&self) -> bool {
        true // middleware already rejected non-members
    }

    pub fn can_edit(&self) -> bool {
        matches!(
            self,
            WorkspaceRole::SuperAdmin | WorkspaceRole::Owner | WorkspaceRole::Editor
        )
    }

    pub fn is_owner(&self) -> bool {
        matches!(self, WorkspaceRole::SuperAdmin | WorkspaceRole::Owner)
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "owner" => Some(WorkspaceRole::Owner),
            "editor" => Some(WorkspaceRole::Editor),
            "viewer" => Some(WorkspaceRole::Viewer),
            _ => None,
        }
    }
}

impl fmt::Display for WorkspaceRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkspaceRole::SuperAdmin => write!(f, "superadmin"),
            WorkspaceRole::Owner => write!(f, "owner"),
            WorkspaceRole::Editor => write!(f, "editor"),
            WorkspaceRole::Viewer => write!(f, "viewer"),
        }
    }
}
