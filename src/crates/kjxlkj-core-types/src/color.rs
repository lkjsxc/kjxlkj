//! Color types for terminal rendering.

use serde::{Deserialize, Serialize};

/// Terminal color representation.
///
/// Supports 16-color (ANSI), 256-color, and 24-bit true color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Color {
    /// Terminal default color.
    Default,
    /// ANSI / 256-color palette index (0..=255).
    Indexed(u8),
    /// 24-bit true color RGB.
    Rgb(u8, u8, u8),
}

impl Color {
    /// Black.
    pub const BLACK: Self = Color::Indexed(0);
    /// Red.
    pub const RED: Self = Color::Indexed(1);
    /// Green.
    pub const GREEN: Self = Color::Indexed(2);
    /// Yellow.
    pub const YELLOW: Self = Color::Indexed(3);
    /// Blue.
    pub const BLUE: Self = Color::Indexed(4);
    /// Magenta.
    pub const MAGENTA: Self = Color::Indexed(5);
    /// Cyan.
    pub const CYAN: Self = Color::Indexed(6);
    /// White.
    pub const WHITE: Self = Color::Indexed(7);
    /// Bright black (gray).
    pub const BRIGHT_BLACK: Self = Color::Indexed(8);
    /// Bright red.
    pub const BRIGHT_RED: Self = Color::Indexed(9);
    /// Bright green.
    pub const BRIGHT_GREEN: Self = Color::Indexed(10);
    /// Bright yellow.
    pub const BRIGHT_YELLOW: Self = Color::Indexed(11);
    /// Bright blue.
    pub const BRIGHT_BLUE: Self = Color::Indexed(12);
    /// Bright magenta.
    pub const BRIGHT_MAGENTA: Self = Color::Indexed(13);
    /// Bright cyan.
    pub const BRIGHT_CYAN: Self = Color::Indexed(14);
    /// Bright white.
    pub const BRIGHT_WHITE: Self = Color::Indexed(15);

    /// Map a 24-bit color to the nearest 256-color index.
    pub fn to_indexed(self) -> Self {
        match self {
            Color::Rgb(r, g, b) => {
                let idx = nearest_256(r, g, b);
                Color::Indexed(idx)
            }
            other => other,
        }
    }

    /// Map a 24-bit color to the nearest 16-color ANSI index.
    pub fn to_ansi(self) -> Self {
        match self {
            Color::Rgb(r, g, b) => {
                let idx = nearest_ansi(r, g, b);
                Color::Indexed(idx)
            }
            Color::Indexed(i) if i >= 16 => {
                let (r, g, b) = index_to_rgb(i);
                let idx = nearest_ansi(r, g, b);
                Color::Indexed(idx)
            }
            other => other,
        }
    }
}

/// Convert a 256-color index to approximate RGB.
fn index_to_rgb(idx: u8) -> (u8, u8, u8) {
    if idx < 16 {
        return ANSI_RGB[idx as usize];
    }
    if idx < 232 {
        let i = idx - 16;
        let b = CUBE_VALS[(i % 6) as usize];
        let g = CUBE_VALS[((i / 6) % 6) as usize];
        let r = CUBE_VALS[(i / 36) as usize];
        return (r, g, b);
    }
    let v = 8 + 10 * (idx - 232);
    (v, v, v)
}

const CUBE_VALS: [u8; 6] = [0, 0x5f, 0x87, 0xaf, 0xd7, 0xff];

const ANSI_RGB: [(u8, u8, u8); 16] = [
    (0, 0, 0),       // 0 black
    (128, 0, 0),     // 1 red
    (0, 128, 0),     // 2 green
    (128, 128, 0),   // 3 yellow
    (0, 0, 128),     // 4 blue
    (128, 0, 128),   // 5 magenta
    (0, 128, 128),   // 6 cyan
    (192, 192, 192), // 7 white
    (128, 128, 128), // 8 bright black
    (255, 0, 0),     // 9 bright red
    (0, 255, 0),     // 10 bright green
    (255, 255, 0),   // 11 bright yellow
    (0, 0, 255),     // 12 bright blue
    (255, 0, 255),   // 13 bright magenta
    (0, 255, 255),   // 14 bright cyan
    (255, 255, 255), // 15 bright white
];

/// Find the nearest 256-color index for an RGB value.
fn nearest_256(r: u8, g: u8, b: u8) -> u8 {
    let mut best = 0u8;
    let mut best_dist = u32::MAX;
    for i in 0..=255u8 {
        let (cr, cg, cb) = index_to_rgb(i);
        let dr = (r as i32 - cr as i32).unsigned_abs();
        let dg = (g as i32 - cg as i32).unsigned_abs();
        let db = (b as i32 - cb as i32).unsigned_abs();
        let dist = dr * dr + dg * dg + db * db;
        if dist < best_dist {
            best_dist = dist;
            best = i;
        }
    }
    best
}

/// Find the nearest 16-color ANSI index for an RGB value.
fn nearest_ansi(r: u8, g: u8, b: u8) -> u8 {
    let mut best = 0u8;
    let mut best_dist = u32::MAX;
    for (i, &(cr, cg, cb)) in ANSI_RGB.iter().enumerate() {
        let dr = (r as i32 - cr as i32).unsigned_abs();
        let dg = (g as i32 - cg as i32).unsigned_abs();
        let db = (b as i32 - cb as i32).unsigned_abs();
        let dist = dr * dr + dg * dg + db * db;
        if dist < best_dist {
            best_dist = dist;
            best = i as u8;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_color() {
        assert_eq!(Color::Default, Color::Default);
    }

    #[test]
    fn rgb_to_indexed() {
        let c = Color::Rgb(255, 0, 0).to_indexed();
        assert!(matches!(c, Color::Indexed(_)));
    }

    #[test]
    fn ansi_constants() {
        assert_eq!(Color::RED, Color::Indexed(1));
        assert_eq!(Color::BRIGHT_WHITE, Color::Indexed(15));
    }
}
