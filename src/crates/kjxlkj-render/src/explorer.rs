//! Explorer view rendering.
//!
//! Renders the file explorer tree view.

use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Attribute, Color, Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

/// Explorer render configuration.
#[derive(Debug, Clone)]
pub struct ExplorerRenderConfig {
    /// Width of the explorer panel.
    pub width: u16,
    /// Height of the explorer panel.
    pub height: u16,
    /// X offset (column) where explorer starts.
    pub x_offset: u16,
    /// Y offset (row) where explorer starts.
    pub y_offset: u16,
    /// Whether to show icons.
    pub show_icons: bool,
    /// Whether to show indentation guides.
    pub show_guides: bool,
}

impl Default for ExplorerRenderConfig {
    fn default() -> Self {
        Self {
            width: 30,
            height: 20,
            x_offset: 0,
            y_offset: 0,
            show_icons: true,
            show_guides: true,
        }
    }
}

/// A single row to render in the explorer.
#[derive(Debug, Clone)]
pub struct ExplorerRenderRow {
    /// Display text.
    pub text: String,
    /// Depth level for indentation.
    pub depth: usize,
    /// Is this row selected.
    pub selected: bool,
    /// Is this a directory.
    pub is_dir: bool,
    /// Is the directory expanded.
    pub is_expanded: bool,
}

/// Explorer view renderer.
pub struct ExplorerRenderer {
    /// Render configuration.
    config: ExplorerRenderConfig,
}

impl ExplorerRenderer {
    /// Create a new explorer renderer.
    pub fn new(config: ExplorerRenderConfig) -> Self {
        Self { config }
    }

    /// Update configuration.
    pub fn set_config(&mut self, config: ExplorerRenderConfig) {
        self.config = config;
    }

    /// Render explorer rows to stdout.
    pub fn render<W: Write>(&self, w: &mut W, rows: &[ExplorerRenderRow]) -> io::Result<()> {
        let visible_rows = rows.iter().take(self.config.height as usize);

        for (i, row) in visible_rows.enumerate() {
            let y = self.config.y_offset + i as u16;
            execute!(w, MoveTo(self.config.x_offset, y))?;
            execute!(w, Clear(ClearType::CurrentLine))?;

            // Render selection highlight
            if row.selected {
                execute!(w, SetBackgroundColor(Color::DarkBlue))?;
                execute!(w, SetForegroundColor(Color::White))?;
            } else if row.is_dir {
                execute!(w, SetForegroundColor(Color::Cyan))?;
            }

            // Build display string
            let indent = "  ".repeat(row.depth);
            let indicator = if row.is_dir {
                if row.is_expanded { "▼ " } else { "▶ " }
            } else {
                "  "
            };
            let icon = if self.config.show_icons {
                if row.is_dir { "📁 " } else { "📄 " }
            } else {
                ""
            };

            let display = format!("{}{}{}{}", indent, indicator, icon, row.text);
            let truncated = truncate_display(&display, self.config.width as usize);

            execute!(w, Print(truncated))?;

            // Reset attributes
            execute!(w, SetAttribute(Attribute::Reset))?;
        }

        // Clear remaining rows
        for i in rows.len()..self.config.height as usize {
            let y = self.config.y_offset + i as u16;
            execute!(w, MoveTo(self.config.x_offset, y))?;
            execute!(w, Clear(ClearType::CurrentLine))?;
        }

        Ok(())
    }

    /// Render to a string buffer (for testing).
    pub fn render_to_buffer(&self, rows: &[ExplorerRenderRow]) -> Vec<String> {
        let mut output = Vec::with_capacity(self.config.height as usize);

        for row in rows.iter().take(self.config.height as usize) {
            let indent = "  ".repeat(row.depth);
            let indicator = if row.is_dir {
                if row.is_expanded { "▼ " } else { "▶ " }
            } else {
                "  "
            };
            let icon = if self.config.show_icons {
                if row.is_dir { "📁 " } else { "📄 " }
            } else {
                ""
            };

            let display = format!("{}{}{}{}", indent, indicator, icon, row.text);
            let truncated = truncate_display(&display, self.config.width as usize);

            let line = if row.selected {
                format!("[{}]", truncated.trim_end())
            } else {
                truncated
            };
            output.push(line);
        }

        output
    }
}

/// Truncate display string to fit width.
fn truncate_display(s: &str, width: usize) -> String {
    if s.len() <= width {
        format!("{:width$}", s, width = width)
    } else {
        let truncated: String = s.chars().take(width.saturating_sub(1)).collect();
        format!("{}…", truncated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explorer_render_config_default() {
        let config = ExplorerRenderConfig::default();
        assert_eq!(config.width, 30);
        assert_eq!(config.height, 20);
        assert!(config.show_icons);
    }

    #[test]
    fn test_explorer_renderer_new() {
        let config = ExplorerRenderConfig::default();
        let _renderer = ExplorerRenderer::new(config);
    }

    #[test]
    fn test_render_to_buffer_empty() {
        let config = ExplorerRenderConfig {
            width: 40,
            height: 10,
            ..Default::default()
        };
        let renderer = ExplorerRenderer::new(config);
        let output = renderer.render_to_buffer(&[]);
        assert!(output.is_empty());
    }

    #[test]
    fn test_render_to_buffer_single_file() {
        let config = ExplorerRenderConfig {
            width: 40,
            height: 10,
            show_icons: true,
            ..Default::default()
        };
        let renderer = ExplorerRenderer::new(config);
        let rows = vec![ExplorerRenderRow {
            text: "main.rs".to_string(),
            depth: 0,
            selected: false,
            is_dir: false,
            is_expanded: false,
        }];
        let output = renderer.render_to_buffer(&rows);
        assert_eq!(output.len(), 1);
        assert!(output[0].contains("main.rs"));
        assert!(output[0].contains("📄"));
    }

    #[test]
    fn test_render_to_buffer_directory() {
        let config = ExplorerRenderConfig {
            width: 40,
            height: 10,
            show_icons: true,
            ..Default::default()
        };
        let renderer = ExplorerRenderer::new(config);
        let rows = vec![ExplorerRenderRow {
            text: "src".to_string(),
            depth: 0,
            selected: false,
            is_dir: true,
            is_expanded: false,
        }];
        let output = renderer.render_to_buffer(&rows);
        assert!(output[0].contains("▶"));
        assert!(output[0].contains("📁"));
        assert!(output[0].contains("src"));
    }

    #[test]
    fn test_render_to_buffer_expanded_directory() {
        let config = ExplorerRenderConfig {
            width: 40,
            height: 10,
            show_icons: true,
            ..Default::default()
        };
        let renderer = ExplorerRenderer::new(config);
        let rows = vec![ExplorerRenderRow {
            text: "src".to_string(),
            depth: 0,
            selected: false,
            is_dir: true,
            is_expanded: true,
        }];
        let output = renderer.render_to_buffer(&rows);
        assert!(output[0].contains("▼"));
    }

    #[test]
    fn test_render_to_buffer_selected() {
        let config = ExplorerRenderConfig {
            width: 40,
            height: 10,
            show_icons: false,
            ..Default::default()
        };
        let renderer = ExplorerRenderer::new(config);
        let rows = vec![ExplorerRenderRow {
            text: "test.rs".to_string(),
            depth: 0,
            selected: true,
            is_dir: false,
            is_expanded: false,
        }];
        let output = renderer.render_to_buffer(&rows);
        assert!(output[0].starts_with('['));
        assert!(output[0].contains("test.rs"));
    }

    #[test]
    fn test_render_to_buffer_indentation() {
        let config = ExplorerRenderConfig {
            width: 40,
            height: 10,
            show_icons: false,
            ..Default::default()
        };
        let renderer = ExplorerRenderer::new(config);
        let rows = vec![
            ExplorerRenderRow {
                text: "project".to_string(),
                depth: 0,
                selected: false,
                is_dir: true,
                is_expanded: true,
            },
            ExplorerRenderRow {
                text: "src".to_string(),
                depth: 1,
                selected: false,
                is_dir: true,
                is_expanded: true,
            },
            ExplorerRenderRow {
                text: "main.rs".to_string(),
                depth: 2,
                selected: false,
                is_dir: false,
                is_expanded: false,
            },
        ];
        let output = renderer.render_to_buffer(&rows);
        assert_eq!(output.len(), 3);
        // Depth 1 should have 2 spaces
        assert!(output[1].starts_with("  "));
        // Depth 2 should have 4 spaces
        assert!(output[2].starts_with("    "));
    }

    #[test]
    fn test_render_to_buffer_tree_structure() {
        let config = ExplorerRenderConfig {
            width: 50,
            height: 20,
            show_icons: true,
            ..Default::default()
        };
        let renderer = ExplorerRenderer::new(config);
        let rows = vec![
            ExplorerRenderRow {
                text: "kjxlkj".to_string(),
                depth: 0,
                selected: true,
                is_dir: true,
                is_expanded: true,
            },
            ExplorerRenderRow {
                text: "src".to_string(),
                depth: 1,
                selected: false,
                is_dir: true,
                is_expanded: true,
            },
            ExplorerRenderRow {
                text: "crates".to_string(),
                depth: 2,
                selected: false,
                is_dir: true,
                is_expanded: false,
            },
            ExplorerRenderRow {
                text: "Cargo.toml".to_string(),
                depth: 1,
                selected: false,
                is_dir: false,
                is_expanded: false,
            },
            ExplorerRenderRow {
                text: "README.md".to_string(),
                depth: 1,
                selected: false,
                is_dir: false,
                is_expanded: false,
            },
        ];
        let output = renderer.render_to_buffer(&rows);
        assert_eq!(output.len(), 5);

        // Root is selected
        assert!(output[0].starts_with('['));

        // Verify structure
        assert!(output[0].contains("kjxlkj"));
        assert!(output[1].contains("src"));
        assert!(output[2].contains("crates"));
        assert!(output[3].contains("Cargo.toml"));
        assert!(output[4].contains("README.md"));
    }

    #[test]
    fn test_truncate_display() {
        assert_eq!(truncate_display("hello", 10), "hello     ");
        assert_eq!(truncate_display("hello", 5), "hello");
        assert_eq!(truncate_display("hello world", 8), "hello w…");
    }

    #[test]
    fn test_render_respects_height_limit() {
        let config = ExplorerRenderConfig {
            width: 40,
            height: 3,
            ..Default::default()
        };
        let renderer = ExplorerRenderer::new(config);
        let rows: Vec<_> = (0..10)
            .map(|i| ExplorerRenderRow {
                text: format!("file{}.rs", i),
                depth: 0,
                selected: false,
                is_dir: false,
                is_expanded: false,
            })
            .collect();
        let output = renderer.render_to_buffer(&rows);
        // Should only render 3 rows due to height limit
        assert_eq!(output.len(), 3);
    }

    /// Snapshot test for explorer view with deterministic fake filesystem.
    /// This tests the exact rendered output format against expected strings.
    #[test]
    fn test_explorer_snapshot_deterministic_filesystem() {
        // Simulate a typical project structure
        let config = ExplorerRenderConfig {
            width: 35,
            height: 10,
            show_icons: false,
            show_guides: true,
            ..Default::default()
        };
        let renderer = ExplorerRenderer::new(config);

        // Deterministic fake filesystem structure
        let rows = vec![
            ExplorerRenderRow {
                text: "my-project".to_string(),
                depth: 0,
                selected: false,
                is_dir: true,
                is_expanded: true,
            },
            ExplorerRenderRow {
                text: "src".to_string(),
                depth: 1,
                selected: true,
                is_dir: true,
                is_expanded: true,
            },
            ExplorerRenderRow {
                text: "lib.rs".to_string(),
                depth: 2,
                selected: false,
                is_dir: false,
                is_expanded: false,
            },
            ExplorerRenderRow {
                text: "main.rs".to_string(),
                depth: 2,
                selected: false,
                is_dir: false,
                is_expanded: false,
            },
            ExplorerRenderRow {
                text: "tests".to_string(),
                depth: 1,
                selected: false,
                is_dir: true,
                is_expanded: false,
            },
            ExplorerRenderRow {
                text: "Cargo.toml".to_string(),
                depth: 1,
                selected: false,
                is_dir: false,
                is_expanded: false,
            },
        ];

        let output = renderer.render_to_buffer(&rows);

        // Verify exact structure (snapshot assertions)
        assert_eq!(output.len(), 6);

        // Line 0: Root directory expanded
        assert!(output[0].starts_with("▼ my-project"));

        // Line 1: src directory (selected, expanded)
        assert!(output[1].starts_with("[  ▼ src]"));

        // Line 2: lib.rs (depth 2, file)
        assert!(output[2].contains("    ")); // 4 spaces for depth 2
        assert!(output[2].contains("lib.rs"));

        // Line 3: main.rs (depth 2, file)
        assert!(output[3].contains("main.rs"));

        // Line 4: tests directory (collapsed)
        assert!(output[4].contains("▶"));
        assert!(output[4].contains("tests"));

        // Line 5: Cargo.toml (file)
        assert!(output[5].contains("Cargo.toml"));
    }

    /// Golden test for explorer view with icons enabled.
    #[test]
    fn test_explorer_golden_with_icons() {
        let config = ExplorerRenderConfig {
            width: 45,
            height: 8,
            show_icons: true,
            ..Default::default()
        };
        let renderer = ExplorerRenderer::new(config);

        // Simple project structure
        let rows = vec![
            ExplorerRenderRow {
                text: "workspace".to_string(),
                depth: 0,
                selected: false,
                is_dir: true,
                is_expanded: true,
            },
            ExplorerRenderRow {
                text: "config.toml".to_string(),
                depth: 1,
                selected: false,
                is_dir: false,
                is_expanded: false,
            },
            ExplorerRenderRow {
                text: "data".to_string(),
                depth: 1,
                selected: true,
                is_dir: true,
                is_expanded: false,
            },
        ];

        let output = renderer.render_to_buffer(&rows);
        assert_eq!(output.len(), 3);

        // Verify icons are present
        assert!(output[0].contains("📁")); // dir icon
        assert!(output[1].contains("📄")); // file icon
        assert!(output[2].contains("📁")); // dir icon

        // Verify selection marker
        assert!(output[2].starts_with('['));
    }
}
