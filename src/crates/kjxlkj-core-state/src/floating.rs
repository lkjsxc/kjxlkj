//! Floating window infrastructure.

use kjxlkj_core_types::WindowId;

/// Float anchor point.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatAnchor {
    Editor,
    Cursor,
    Window,
    NW,
    NE,
    SW,
    SE,
}

/// Float border style.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FloatBorder {
    None,
    Single,
    Double,
    Rounded,
    Solid,
    Shadow,
    Custom([String; 8]),
}

impl Default for FloatBorder {
    fn default() -> Self {
        Self::Rounded
    }
}

/// Configuration for creating a floating window.
#[derive(Debug, Clone)]
pub struct FloatConfig {
    pub width: u16,
    pub height: u16,
    pub row: i16,
    pub col: i16,
    pub anchor: FloatAnchor,
    pub border: FloatBorder,
    pub title: Option<String>,
    pub title_pos: TitlePos,
    pub footer: Option<String>,
    pub focusable: bool,
    pub enter: bool,
    pub zindex: u32,
    pub close_on_focus_loss: bool,
    pub center: bool,
    pub relative_width: Option<f32>,
    pub relative_height: Option<f32>,
}

/// Title/footer alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TitlePos {
    Left,
    Center,
    Right,
}

impl Default for FloatConfig {
    fn default() -> Self {
        Self {
            width: 40,
            height: 10,
            row: 0,
            col: 0,
            anchor: FloatAnchor::Editor,
            border: FloatBorder::default(),
            title: None,
            title_pos: TitlePos::Left,
            footer: None,
            focusable: true,
            enter: true,
            zindex: 50,
            close_on_focus_loss: false,
            center: false,
            relative_width: None,
            relative_height: None,
        }
    }
}

/// A floating window state.
#[derive(Debug)]
pub struct FloatingWindow {
    pub window_id: WindowId,
    pub config: FloatConfig,
    pub creation_order: u64,
}

/// Registry of floating windows.
#[derive(Debug, Default)]
pub struct FloatRegistry {
    pub floats: Vec<FloatingWindow>,
    next_order: u64,
}

impl FloatRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a floating window.
    pub fn add(
        &mut self,
        window_id: WindowId,
        config: FloatConfig,
    ) {
        self.next_order += 1;
        self.floats.push(FloatingWindow {
            window_id,
            config,
            creation_order: self.next_order,
        });
    }

    /// Remove a floating window.
    pub fn remove(&mut self, window_id: WindowId) {
        self.floats
            .retain(|f| f.window_id != window_id);
    }

    /// Check if a window is floating.
    pub fn is_floating(
        &self,
        window_id: WindowId,
    ) -> bool {
        self.floats
            .iter()
            .any(|f| f.window_id == window_id)
    }

    /// Get floats in render order (z-index, creation).
    pub fn render_order(
        &self,
    ) -> Vec<&FloatingWindow> {
        let mut ordered: Vec<&FloatingWindow> =
            self.floats.iter().collect();
        ordered.sort_by_key(|f| {
            (f.config.zindex, f.creation_order)
        });
        ordered
    }

    /// Get focusable floats for cycling.
    pub fn focusable_ids(&self) -> Vec<WindowId> {
        self.floats
            .iter()
            .filter(|f| f.config.focusable)
            .map(|f| f.window_id)
            .collect()
    }

    /// Compute positioned rect for a float.
    pub fn compute_rect(
        config: &FloatConfig,
        term_cols: u16,
        term_rows: u16,
    ) -> (u16, u16, u16, u16) {
        let w = if let Some(rel) =
            config.relative_width
        {
            ((term_cols as f32) * rel) as u16
        } else {
            config.width
        };
        let h = if let Some(rel) =
            config.relative_height
        {
            ((term_rows as f32) * rel) as u16
        } else {
            config.height
        };
        let (x, y) = if config.center {
            (
                term_cols.saturating_sub(w) / 2,
                term_rows.saturating_sub(h) / 2,
            )
        } else {
            (
                config.col.max(0) as u16,
                config.row.max(0) as u16,
            )
        };
        let x = x.min(term_cols.saturating_sub(w));
        let y = y.min(term_rows.saturating_sub(h));
        (x, y, w, h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_centered() {
        let config = FloatConfig {
            width: 20,
            height: 10,
            center: true,
            ..Default::default()
        };
        let (x, y, w, h) =
            FloatRegistry::compute_rect(
                &config, 80, 24,
            );
        assert_eq!(w, 20);
        assert_eq!(h, 10);
        assert_eq!(x, 30);
        assert_eq!(y, 7);
    }

    #[test]
    fn render_order_by_zindex() {
        let mut reg = FloatRegistry::new();
        let w1 = WindowId(1);
        let w2 = WindowId(2);
        reg.add(
            w1,
            FloatConfig {
                zindex: 100,
                ..Default::default()
            },
        );
        reg.add(w2, FloatConfig::default());
        let order = reg.render_order();
        assert_eq!(order[0].window_id, w2);
        assert_eq!(order[1].window_id, w1);
    }
}
