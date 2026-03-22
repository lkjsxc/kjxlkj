use serde::Deserialize;

use crate::core::content::slug_from_stem;

#[derive(Debug, Deserialize)]
pub struct CreateForm {
    pub slug: String,
    pub title: Option<String>,
    pub body: String,
    pub private: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SaveForm {
    pub slug: String,
    pub title: Option<String>,
    pub body: String,
    pub private: Option<bool>,
    pub last_known_revision: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RenameForm {
    pub slug: String,
    pub new_slug: String,
}

pub fn normalize_slug_input(value: &str, field_name: &str) -> Result<String, String> {
    let Some(slug) = normalize_required(value) else {
        return Err(format!("{field_name} is required"));
    };
    if slug_from_stem(&slug).is_err() {
        return Err(format!("{field_name} must be lowercase kebab-case"));
    }
    Ok(slug)
}

fn normalize_required(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}
