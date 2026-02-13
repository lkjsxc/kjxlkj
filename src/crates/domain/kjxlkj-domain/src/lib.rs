use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Owner,
    Admin,
    Editor,
    Viewer,
}

impl Role {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Owner => "owner",
            Self::Admin => "admin",
            Self::Editor => "editor",
            Self::Viewer => "viewer",
        }
    }

    pub fn can_manage_global_roles(self) -> bool {
        matches!(self, Self::Owner)
    }

    pub fn can_manage_workspace_members(self) -> bool {
        matches!(self, Self::Owner | Self::Admin)
    }

    pub fn can_manage_automation(self) -> bool {
        matches!(self, Self::Owner | Self::Admin)
    }

    pub fn can_view_workspace(self) -> bool {
        matches!(self, Self::Owner | Self::Admin | Self::Editor | Self::Viewer)
    }

    pub fn can_write_notes(self) -> bool {
        matches!(self, Self::Owner | Self::Admin | Self::Editor)
    }
}

#[derive(Debug, Error, Clone)]
#[error("invalid role '{value}'")]
pub struct ParseRoleError {
    pub value: String,
}

impl FromStr for Role {
    type Err = ParseRoleError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "owner" => Ok(Self::Owner),
            "admin" => Ok(Self::Admin),
            "editor" => Ok(Self::Editor),
            "viewer" => Ok(Self::Viewer),
            _ => Err(ParseRoleError {
                value: value.to_owned(),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Disabled,
}

impl UserStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Disabled => "disabled",
        }
    }
}
