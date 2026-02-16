use serde::{Deserialize, Serialize};

/// Role set per docs/spec/domain/permissions.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Viewer = 0,
    Editor = 1,
    Admin = 2,
    Owner = 3,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Viewer => "viewer",
            Self::Editor => "editor",
            Self::Admin => "admin",
            Self::Owner => "owner",
        }
    }

    pub fn from_str_checked(s: &str) -> Option<Self> {
        match s {
            "viewer" => Some(Self::Viewer),
            "editor" => Some(Self::Editor),
            "admin" => Some(Self::Admin),
            "owner" => Some(Self::Owner),
            _ => None,
        }
    }

    /// Check if this role can perform write operations on notes.
    pub fn can_write_notes(&self) -> bool {
        *self >= Self::Editor
    }

    /// Check if this role can manage members and settings.
    pub fn can_manage(&self) -> bool {
        *self >= Self::Admin
    }

    /// Check if this role has full ownership control.
    pub fn is_owner(&self) -> bool {
        *self == Self::Owner
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
