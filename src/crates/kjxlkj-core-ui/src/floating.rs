//! Floating window types, layout presets, resize operations, and window commands.

use crate::layout::Rect;
use serde::{Deserialize, Serialize};

/// Border style for floating windows.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FloatBorder {
    None,
    Single,
    Double,
    Rounded,
    Solid,
    Shadow,
}

/// Anchor point for positioning a floating window.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FloatAnchor {
    NW,
    NE,
    SW,
    SE,
    Center,
}

/// Configuration for a floating window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatConfig {
    pub anchor: FloatAnchor,
    pub width: u16,
    pub height: u16,
    pub border: FloatBorder,
    pub row_offset: i16,
    pub col_offset: i16,
    pub zindex: u16,
}

/// Zoom state of a window.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZoomState {
    Normal,
    Zoomed { restore_w: u16, restore_h: u16 },
}

/// Predefined layout presets for arranging multiple windows.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LayoutPreset {
    Single,
    EqualH,
    EqualV,
    MainLeft,
    MainTop,
    Grid,
}

/// Compute rectangles for a given layout preset.
pub fn compute_preset_rects(preset: LayoutPreset, w: u16, h: u16, count: usize) -> Vec<Rect> {
    if count == 0 {
        return Vec::new();
    }
    match preset {
        LayoutPreset::Single => vec![Rect::new(0, 0, w, h)],
        LayoutPreset::EqualH => {
            let each = h / count.max(1) as u16;
            (0..count)
                .map(|i| {
                    let y = i as u16 * each;
                    let eh = if i == count - 1 { h - y } else { each };
                    Rect::new(0, y, w, eh)
                })
                .collect()
        }
        LayoutPreset::EqualV => {
            let each = w / count.max(1) as u16;
            (0..count)
                .map(|i| {
                    let x = i as u16 * each;
                    let ew = if i == count - 1 { w - x } else { each };
                    Rect::new(x, 0, ew, h)
                })
                .collect()
        }
        LayoutPreset::MainLeft => {
            let main_w = w / 2;
            let mut rects = vec![Rect::new(0, 0, main_w, h)];
            let side = count.saturating_sub(1).max(1);
            let each = h / side as u16;
            for i in 0..count.saturating_sub(1) {
                let y = i as u16 * each;
                let eh = if i == side - 1 { h - y } else { each };
                rects.push(Rect::new(main_w, y, w - main_w, eh));
            }
            rects
        }
        LayoutPreset::MainTop => {
            let main_h = h / 2;
            let mut rects = vec![Rect::new(0, 0, w, main_h)];
            let bottom = count.saturating_sub(1).max(1);
            let each = w / bottom as u16;
            for i in 0..count.saturating_sub(1) {
                let x = i as u16 * each;
                let ew = if i == bottom - 1 { w - x } else { each };
                rects.push(Rect::new(x, main_h, ew, h - main_h));
            }
            rects
        }
        LayoutPreset::Grid => {
            let cols = (count as f64).sqrt().ceil() as u16;
            let rows = ((count as u16) + cols - 1) / cols;
            let cw = w / cols;
            let rh = h / rows;
            (0..count)
                .map(|i| {
                    let c = i as u16 % cols;
                    let r = i as u16 / cols;
                    let x = c * cw;
                    let y = r * rh;
                    let ew = if c == cols - 1 { w - x } else { cw };
                    let eh = if r == rows - 1 { h - y } else { rh };
                    Rect::new(x, y, ew, eh)
                })
                .collect()
        }
    }
}

/// Resize operation for a window.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResizeOp {
    IncrementH(i16),
    IncrementW(i16),
    SetH(u16),
    SetW(u16),
    Maximize,
    Equalize,
}

/// Window management command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WinCmd {
    SplitH,
    SplitV,
    Close,
    FocusLeft,
    FocusRight,
    FocusUp,
    FocusDown,
    Rotate,
    Exchange,
    Resize(ResizeOp),
    Zoom,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_preset() {
        let rects = compute_preset_rects(LayoutPreset::Single, 80, 24, 1);
        assert_eq!(rects.len(), 1);
        assert_eq!(rects[0], Rect::new(0, 0, 80, 24));
    }

    #[test]
    fn equal_h_preset() {
        let rects = compute_preset_rects(LayoutPreset::EqualH, 80, 24, 3);
        assert_eq!(rects.len(), 3);
        assert_eq!(rects[0].y, 0);
    }

    #[test]
    fn equal_v_preset() {
        let rects = compute_preset_rects(LayoutPreset::EqualV, 80, 24, 2);
        assert_eq!(rects.len(), 2);
        assert_eq!(rects[0].w, 40);
    }

    #[test]
    fn grid_preset() {
        let rects = compute_preset_rects(LayoutPreset::Grid, 80, 24, 4);
        assert_eq!(rects.len(), 4);
    }

    #[test]
    fn empty_count() {
        assert!(compute_preset_rects(LayoutPreset::Single, 80, 24, 0).is_empty());
    }
}
