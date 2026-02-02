//! Cursor appearance and shape configuration.

use serde::{Deserialize, Serialize};

pub use crate::cursor_blink::CursorBlink;

/// Cursor shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CursorShape {
    /// Block cursor.
    Block,
    /// Underline cursor.
    Underline,
    /// Vertical bar cursor.
    Bar,
}

impl Default for CursorShape {
    fn default() -> Self {
        Self::Block
    }
}

/// Cursor appearance configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorAppearance {
    /// Shape in normal mode.
    pub normal_shape: CursorShape,
    /// Shape in insert mode.
    pub insert_shape: CursorShape,
    /// Shape in replace mode.
    pub replace_shape: CursorShape,
    /// Shape in visual mode.
    pub visual_shape: CursorShape,
}

impl Default for CursorAppearance {
    fn default() -> Self {
        Self {
            normal_shape: CursorShape::Block,
            insert_shape: CursorShape::Bar,
            replace_shape: CursorShape::Underline,
            visual_shape: CursorShape::Block,
        }
    }
}

impl CursorAppearance {
    /// Creates a new cursor appearance config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns shape for a mode name.
    pub fn shape_for_mode(&self, mode: &str) -> CursorShape {
        match mode.to_lowercase().as_str() {
            "normal" => self.normal_shape,
            "insert" => self.insert_shape,
            "replace" => self.replace_shape,
            "visual" | "visualline" | "visualblock" => self.visual_shape,
            _ => self.normal_shape,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_shape_default() {
        assert_eq!(CursorShape::default(), CursorShape::Block);
    }

    #[test]
    fn test_cursor_appearance_default() {
        let app = CursorAppearance::default();
        assert_eq!(app.normal_shape, CursorShape::Block);
        assert_eq!(app.insert_shape, CursorShape::Bar);
        assert_eq!(app.replace_shape, CursorShape::Underline);
    }

    #[test]
    fn test_cursor_appearance_shape_for_mode() {
        let app = CursorAppearance::default();
        assert_eq!(app.shape_for_mode("normal"), CursorShape::Block);
        assert_eq!(app.shape_for_mode("insert"), CursorShape::Bar);
        assert_eq!(app.shape_for_mode("visual"), CursorShape::Block);
    }
}
