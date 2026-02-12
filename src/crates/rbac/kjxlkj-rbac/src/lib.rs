use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Owner,
    Admin,
    Editor,
    Viewer,
}

pub fn can_mutate(role: Role) -> bool {
    matches!(role, Role::Owner | Role::Admin | Role::Editor)
}
