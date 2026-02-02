//! Syntax highlighting groups.
//!
//! Named highlight groups for syntax highlighting.

use std::collections::HashMap;

/// RGB color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    /// Red component.
    pub r: u8,
    /// Green component.
    pub g: u8,
    /// Blue component.
    pub b: u8,
}

impl Color {
    /// Creates a new color.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Creates from hex value.
    pub const fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }
}

/// Text attributes.
#[derive(Debug, Clone, Copy, Default)]
pub struct TextAttr {
    /// Bold.
    pub bold: bool,
    /// Italic.
    pub italic: bool,
    /// Underline.
    pub underline: bool,
    /// Strikethrough.
    pub strikethrough: bool,
}

/// Highlight group.
#[derive(Debug, Clone)]
pub struct HlGroup {
    /// Group name.
    pub name: String,
    /// Foreground color.
    pub fg: Option<Color>,
    /// Background color.
    pub bg: Option<Color>,
    /// Text attributes.
    pub attr: TextAttr,
    /// Linked group.
    pub link: Option<String>,
}

impl HlGroup {
    /// Creates a new group.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            fg: None,
            bg: None,
            attr: TextAttr::default(),
            link: None,
        }
    }

    /// Sets foreground color.
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Sets background color.
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Sets bold attribute.
    pub fn bold(mut self) -> Self {
        self.attr.bold = true;
        self
    }

    /// Links to another group.
    pub fn link_to(mut self, target: &str) -> Self {
        self.link = Some(target.to_string());
        self
    }
}

/// Highlight group manager.
#[derive(Debug, Default)]
pub struct HlGroups {
    /// Groups by name.
    groups: HashMap<String, HlGroup>,
}

impl HlGroups {
    /// Creates a new manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Defines a group.
    pub fn define(&mut self, group: HlGroup) {
        self.groups.insert(group.name.clone(), group);
    }

    /// Gets a group.
    pub fn get(&self, name: &str) -> Option<&HlGroup> {
        self.groups.get(name)
    }

    /// Resolves a group (follows links).
    pub fn resolve(&self, name: &str) -> Option<&HlGroup> {
        let mut current = self.groups.get(name)?;
        for _ in 0..10 {
            // Limit to prevent cycles
            if let Some(ref link) = current.link {
                if let Some(linked) = self.groups.get(link) {
                    current = linked;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Some(current)
    }

    /// Clears a group.
    pub fn clear(&mut self, name: &str) {
        self.groups.remove(name);
    }

    /// Lists all group names.
    pub fn names(&self) -> Vec<&str> {
        self.groups.keys().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_new() {
        let c = Color::new(255, 128, 0);
        assert_eq!(c.r, 255);
    }

    #[test]
    fn test_color_from_hex() {
        let c = Color::from_hex(0xFF8000);
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 128);
    }

    #[test]
    fn test_hl_group_new() {
        let g = HlGroup::new("Comment").fg(Color::new(100, 100, 100));
        assert!(g.fg.is_some());
    }

    #[test]
    fn test_hl_groups_define() {
        let mut groups = HlGroups::new();
        groups.define(HlGroup::new("String"));
        assert!(groups.get("String").is_some());
    }

    #[test]
    fn test_hl_groups_resolve_link() {
        let mut groups = HlGroups::new();
        groups.define(HlGroup::new("Comment").fg(Color::new(100, 100, 100)));
        groups.define(HlGroup::new("LineComment").link_to("Comment"));
        let resolved = groups.resolve("LineComment").unwrap();
        assert!(resolved.fg.is_some());
    }

    #[test]
    fn test_hl_groups_names() {
        let mut groups = HlGroups::new();
        groups.define(HlGroup::new("A"));
        groups.define(HlGroup::new("B"));
        assert_eq!(groups.names().len(), 2);
    }
}
