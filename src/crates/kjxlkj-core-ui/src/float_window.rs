//! Floating window, layout presets, zoom, and resize types.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatBorder { None, Single, Double, Rounded, Solid, Shadow }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatAnchor { NW, NE, SW, SE, Center }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatSize { Fixed(u16, u16), Percent(u8, u8) }

#[derive(Debug, Clone)]
pub struct FloatConfig {
    pub anchor: FloatAnchor, pub size: FloatSize, pub border: FloatBorder,
    pub row: i16, pub col: i16, pub focusable: bool, pub zindex: u16,
}

impl FloatConfig {
    pub fn new(size: FloatSize) -> Self {
        Self { anchor: FloatAnchor::NW, size, border: FloatBorder::Single,
               row: 0, col: 0, focusable: true, zindex: 50 }
    }
    pub fn resolve_size(&self, pw: u16, ph: u16) -> (u16, u16) {
        match self.size {
            FloatSize::Fixed(w, h) => (w.min(pw), h.min(ph)),
            FloatSize::Percent(wp, hp) => (
                (pw as u32 * wp.min(100) as u32 / 100) as u16,
                (ph as u32 * hp.min(100) as u32 / 100) as u16,
            ),
        }
    }
    pub fn resolve_position(&self, pw: u16, ph: u16) -> (u16, u16) {
        let (w, h) = self.resolve_size(pw, ph);
        let (bx, by) = match self.anchor {
            FloatAnchor::NW => (0u16, 0u16), FloatAnchor::NE => (pw.saturating_sub(w), 0),
            FloatAnchor::SW => (0, ph.saturating_sub(h)),
            FloatAnchor::SE => (pw.saturating_sub(w), ph.saturating_sub(h)),
            FloatAnchor::Center => (pw.saturating_sub(w) / 2, ph.saturating_sub(h) / 2),
        };
        ((bx as i32 + self.col as i32).clamp(0, pw.saturating_sub(1) as i32) as u16,
         (by as i32 + self.row as i32).clamp(0, ph.saturating_sub(1) as i32) as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoomState { Normal, Zoomed { prev_height: usize, prev_width: usize } }
impl ZoomState {
    pub fn is_zoomed(&self) -> bool { matches!(self, ZoomState::Zoomed { .. }) }
    pub fn toggle(self, h: usize, w: usize) -> Self {
        match self { Self::Normal => Self::Zoomed { prev_height: h, prev_width: w }, Self::Zoomed { .. } => Self::Normal }
    }
    pub fn restore_size(&self) -> Option<(usize, usize)> {
        match self { Self::Zoomed { prev_height, prev_width } => Some((*prev_height, *prev_width)), _ => None }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutPreset { Single, EqualHorizontal, EqualVertical, MainLeft, MainTop, Grid }
impl LayoutPreset {
    pub fn compute(&self, n: usize, total_w: u16, total_h: u16) -> Vec<(u16, u16, u16, u16)> {
        if n == 0 { return Vec::new(); }
        if n == 1 { return vec![(0, 0, total_w, total_h)]; }
        match self {
            Self::Single => vec![(0, 0, total_w, total_h)],
            Self::EqualHorizontal => {
                let each = total_h / n as u16;
                (0..n).map(|i| (0, i as u16 * each, total_w, each)).collect()
            }
            Self::EqualVertical => {
                let each = total_w / n as u16;
                (0..n).map(|i| (i as u16 * each, 0, each, total_h)).collect()
            }
            Self::MainLeft => {
                let (mw, sw) = (total_w * 2 / 3, total_w - total_w * 2 / 3);
                let sh = total_h / (n - 1).max(1) as u16;
                let mut o = vec![(0, 0, mw, total_h)];
                for i in 0..(n-1) { o.push((mw, i as u16 * sh, sw, sh)); } o
            }
            Self::MainTop => {
                let (mh, bh) = (total_h * 2 / 3, total_h - total_h * 2 / 3);
                let bw = total_w / (n - 1).max(1) as u16;
                let mut o = vec![(0, 0, total_w, mh)];
                for i in 0..(n-1) { o.push((i as u16 * bw, mh, bw, bh)); } o
            }
            Self::Grid => {
                let c = (n as f64).sqrt().ceil() as u16;
                let r = ((n as u16) + c - 1) / c;
                (0..n as u16).map(|i| ((i%c)*(total_w/c), (i/c)*(total_h/r), total_w/c, total_h/r)).collect()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeOp {
    IncrementHeight(i16), IncrementWidth(i16),
    SetHeight(u16), SetWidth(u16),
    Maximize, Equalize,
}

impl ResizeOp {
    pub fn apply(&self, cur_w: u16, cur_h: u16, max_w: u16, max_h: u16) -> (u16, u16) {
        match self {
            Self::IncrementHeight(d) => (cur_w, (cur_h as i32 + *d as i32).clamp(1, max_h as i32) as u16),
            Self::IncrementWidth(d) => ((cur_w as i32 + *d as i32).clamp(1, max_w as i32) as u16, cur_h),
            Self::SetHeight(h) => (cur_w, (*h).min(max_h).max(1)),
            Self::SetWidth(w) => ((*w).min(max_w).max(1), cur_h),
            Self::Maximize => (max_w, max_h),
            Self::Equalize => (cur_w, cur_h),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WinCmd {
    SplitH, SplitV, Close, Only,
    FocusNext, FocusPrev, FocusUp, FocusDown, FocusLeft, FocusRight,
    Rotate, Exchange,
    Resize(ResizeOp),
    ToggleZoom,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_resolve_fixed() {
        let fc = FloatConfig::new(FloatSize::Fixed(40, 10));
        assert_eq!(fc.resolve_size(80, 24), (40, 10));
    }

    #[test]
    fn float_resolve_percent() {
        let fc = FloatConfig::new(FloatSize::Percent(50, 50));
        assert_eq!(fc.resolve_size(80, 24), (40, 12));
    }

    #[test]
    fn float_center_position() {
        let fc = FloatConfig { anchor: FloatAnchor::Center, ..FloatConfig::new(FloatSize::Fixed(20, 10)) };
        let (x, y) = fc.resolve_position(80, 24);
        assert_eq!(x, 30);
        assert_eq!(y, 7);
    }

    #[test]
    fn zoom_toggle() {
        let z = ZoomState::Normal;
        let z2 = z.toggle(24, 80);
        assert!(z2.is_zoomed());
        assert_eq!(z2.restore_size(), Some((24, 80)));
        let z3 = z2.toggle(48, 160);
        assert!(!z3.is_zoomed());
    }

    #[test]
    fn layout_equal_horizontal() {
        let areas = LayoutPreset::EqualHorizontal.compute(3, 80, 24);
        assert_eq!(areas.len(), 3);
        assert_eq!(areas[0], (0, 0, 80, 8));
        assert_eq!(areas[1], (0, 8, 80, 8));
    }

    #[test]
    fn layout_main_left() {
        let areas = LayoutPreset::MainLeft.compute(3, 90, 24);
        assert_eq!(areas.len(), 3);
        assert_eq!(areas[0].2, 60); // main 2/3
        assert_eq!(areas[1].0, 60); // side starts at main width
    }

    #[test]
    fn layout_grid_four() {
        let areas = LayoutPreset::Grid.compute(4, 80, 24);
        assert_eq!(areas.len(), 4);
    }

    #[test]
    fn resize_ops() {
        assert_eq!(ResizeOp::IncrementHeight(5).apply(80, 20, 80, 40), (80, 25));
        assert_eq!(ResizeOp::IncrementHeight(-100).apply(80, 20, 80, 40), (80, 1));
    }

    #[test]
    fn wincmd_and_border_variants() {
        let cmds = [WinCmd::SplitH, WinCmd::Close, WinCmd::ToggleZoom, WinCmd::Resize(ResizeOp::Maximize)];
        let borders = [FloatBorder::None, FloatBorder::Single, FloatBorder::Double,
                        FloatBorder::Rounded, FloatBorder::Solid, FloatBorder::Shadow];
        assert_eq!(cmds.len(), 4);
        assert_eq!(borders.len(), 6);
    }
}
