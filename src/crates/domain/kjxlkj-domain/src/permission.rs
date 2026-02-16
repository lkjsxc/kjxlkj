/// Permission domain types per /docs/spec/domain/permissions.md
use serde::{Deserialize, Serialize};

/// Role set per /docs/spec/domain/permissions.md
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

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "viewer" => Some(Self::Viewer),
            "editor" => Some(Self::Editor),
            "admin" => Some(Self::Admin),
            "owner" => Some(Self::Owner),
            _ => None,
        }
    }

    /// Check if this role can perform write operations
    pub fn can_write(&self) -> bool {
        matches!(self, Self::Editor | Self::Admin | Self::Owner)
    }

    /// Check if this role can manage workspace settings
    pub fn can_manage(&self) -> bool {
        matches!(self, Self::Admin | Self::Owner)
    }

    /// Check if this role has full control
    pub fn is_owner(&self) -> bool {
        matches!(self, Self::Owner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_roundtrip() {
        for role in [Role::Viewer, Role::Editor, Role::Admin, Role::Owner] {
            assert_eq!(Role::from_str(role.as_str()), Some(role));
        }
    }

    #[test]
    fn test_role_capabilities() {
        assert!(!Role::Viewer.can_write());
        assert!(Role::Editor.can_write());
        assert!(Role::Admin.can_manage());
        assert!(Role::Owner.is_owner());
    }
}
