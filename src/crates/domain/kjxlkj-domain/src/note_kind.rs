use serde::{Deserialize, Serialize};

/// Canonical note kind enum per docs/spec/domain/note-types.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteKind {
    Markdown,
    Settings,
    MediaImage,
    MediaVideo,
}

impl NoteKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Markdown => "markdown",
            Self::Settings => "settings",
            Self::MediaImage => "media_image",
            Self::MediaVideo => "media_video",
        }
    }

    pub fn from_str_checked(s: &str) -> Option<Self> {
        match s {
            "markdown" => Some(Self::Markdown),
            "settings" => Some(Self::Settings),
            "media_image" => Some(Self::MediaImage),
            "media_video" => Some(Self::MediaVideo),
            _ => None,
        }
    }
}

impl std::fmt::Display for NoteKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
