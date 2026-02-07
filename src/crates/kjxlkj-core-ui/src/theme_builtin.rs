//! Built-in theme definitions.

use std::collections::HashMap;

use crate::theme::{Theme, ThemePalette};

/// Dark theme (default).
pub fn theme_dark() -> Theme {
    Theme {
        name: "dark".into(),
        palette: ThemePalette {
            fg: "#d4d4d4".into(),
            bg: "#1e1e1e".into(),
            cursor: "#aeafad".into(),
            selection: "#264f78".into(),
            keyword: "#569cd6".into(),
            string: "#ce9178".into(),
            comment: "#6a9955".into(),
            function: "#dcdcaa".into(),
            type_color: "#4ec9b0".into(),
            number: "#b5cea8".into(),
            operator: "#d4d4d4".into(),
            special: "#c586c0".into(),
            error: "#f44747".into(),
            warning: "#cca700".into(),
            info: "#3794ff".into(),
            hint: "#6a9955".into(),
            line_nr: "#858585".into(),
        },
        overrides: HashMap::new(),
    }
}

/// Light theme.
pub fn theme_light() -> Theme {
    Theme {
        name: "light".into(),
        palette: ThemePalette {
            fg: "#333333".into(),
            bg: "#ffffff".into(),
            cursor: "#000000".into(),
            selection: "#add6ff".into(),
            keyword: "#0000ff".into(),
            string: "#a31515".into(),
            comment: "#008000".into(),
            function: "#795e26".into(),
            type_color: "#267f99".into(),
            number: "#098658".into(),
            operator: "#333333".into(),
            special: "#af00db".into(),
            error: "#e51400".into(),
            warning: "#bf8803".into(),
            info: "#1a85ff".into(),
            hint: "#008000".into(),
            line_nr: "#237893".into(),
        },
        overrides: HashMap::new(),
    }
}

/// Gruvbox theme.
pub fn theme_gruvbox() -> Theme {
    Theme {
        name: "gruvbox".into(),
        palette: ThemePalette {
            fg: "#ebdbb2".into(),
            bg: "#282828".into(),
            cursor: "#ebdbb2".into(),
            selection: "#504945".into(),
            keyword: "#fb4934".into(),
            string: "#b8bb26".into(),
            comment: "#928374".into(),
            function: "#fabd2f".into(),
            type_color: "#8ec07c".into(),
            number: "#d3869b".into(),
            operator: "#fe8019".into(),
            special: "#d3869b".into(),
            error: "#fb4934".into(),
            warning: "#fabd2f".into(),
            info: "#83a598".into(),
            hint: "#928374".into(),
            line_nr: "#665c54".into(),
        },
        overrides: HashMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dark_has_name() {
        assert_eq!(theme_dark().name, "dark");
    }

    #[test]
    fn light_has_name() {
        assert_eq!(theme_light().name, "light");
    }

    #[test]
    fn gruvbox_has_name() {
        assert_eq!(theme_gruvbox().name, "gruvbox");
    }

    #[test]
    fn palettes_differ() {
        assert_ne!(theme_dark().palette.bg, theme_light().palette.bg);
    }
}
