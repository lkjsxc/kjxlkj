//! Notification rendering and display layout.
//!
//! Computes display dimensions and positioning for notification messages
//! shown briefly in the editor UI.

/// Position for notification display.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotifPosition {
    TopRight,
    BottomRight,
    TopCenter,
    BottomCenter,
}

/// Rendered notification ready for display.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedNotif {
    pub text: String,
    pub row: u16,
    pub col: u16,
    pub width: u16,
    pub height: u16,
    pub is_error: bool,
}

/// Compute the rendered position of a notification.
pub fn render_notification(
    text: &str,
    position: NotifPosition,
    screen_width: u16,
    screen_height: u16,
    is_error: bool,
    index: usize,
) -> RenderedNotif {
    let max_width = (screen_width / 3).max(20).min(60);
    let lines = wrap_text(text, max_width as usize);
    let width = lines.iter().map(|l| l.len() as u16).max().unwrap_or(0).min(max_width);
    let height = lines.len() as u16;
    let wrapped = lines.join("\n");
    let offset = index as u16 * (height + 1);
    let (row, col) = match position {
        NotifPosition::TopRight => (1 + offset, screen_width.saturating_sub(width + 2)),
        NotifPosition::BottomRight => (
            screen_height.saturating_sub(height + 2 + offset),
            screen_width.saturating_sub(width + 2),
        ),
        NotifPosition::TopCenter => (1 + offset, screen_width / 2 - width / 2),
        NotifPosition::BottomCenter => (
            screen_height.saturating_sub(height + 2 + offset),
            screen_width / 2 - width / 2,
        ),
    };
    RenderedNotif { text: wrapped, row, col, width, height, is_error }
}

/// Wrap text to fit within max_width columns.
fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 { return vec![text.to_string()]; }
    let mut lines = Vec::new();
    for line in text.lines() {
        if line.len() <= max_width {
            lines.push(line.to_string());
        } else {
            let mut remaining = line;
            while remaining.len() > max_width {
                let split = find_wrap_point(remaining, max_width);
                lines.push(remaining[..split].to_string());
                remaining = remaining[split..].trim_start();
            }
            if !remaining.is_empty() { lines.push(remaining.to_string()); }
        }
    }
    if lines.is_empty() { lines.push(String::new()); }
    lines
}

/// Find a good wrap point (prefer space).
fn find_wrap_point(text: &str, max: usize) -> usize {
    if let Some(pos) = text[..max].rfind(' ') {
        if pos > max / 3 { return pos + 1; }
    }
    max
}

/// Compute how many notifications can fit on screen.
pub fn max_visible_notifications(screen_height: u16, avg_height: u16) -> usize {
    (screen_height / (avg_height + 1).max(1)) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_top_right() {
        let r = render_notification("Hello", NotifPosition::TopRight, 80, 24, false, 0);
        assert_eq!(r.row, 1);
        assert!(r.col > 50);
        assert_eq!(r.height, 1);
    }

    #[test]
    fn render_bottom_right() {
        let r = render_notification("Error!", NotifPosition::BottomRight, 80, 24, true, 0);
        assert!(r.row > 15);
        assert!(r.is_error);
    }

    #[test]
    fn render_stacked() {
        let r0 = render_notification("First", NotifPosition::TopRight, 80, 24, false, 0);
        let r1 = render_notification("Second", NotifPosition::TopRight, 80, 24, false, 1);
        assert!(r1.row > r0.row);
    }

    #[test]
    fn wrap_long_text() {
        let lines = wrap_text("This is a long notification message that should wrap", 20);
        assert!(lines.len() > 1);
        for line in &lines { assert!(line.len() <= 20); }
    }

    #[test]
    fn wrap_short_text() {
        let lines = wrap_text("Short", 80);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "Short");
    }

    #[test]
    fn max_visible() {
        assert_eq!(max_visible_notifications(24, 2), 8);
    }

    #[test]
    fn render_center() {
        let r = render_notification("Centered", NotifPosition::TopCenter, 80, 24, false, 0);
        assert!(r.col > 30 && r.col < 50);
    }
}
