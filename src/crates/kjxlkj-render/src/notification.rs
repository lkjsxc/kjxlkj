//! Notification rendering and positioning.

use serde::{Deserialize, Serialize};

/// Screen position for notifications.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotifPosition {
    TopRight,
    BottomRight,
    TopCenter,
    BottomCenter,
}

/// A rendered notification ready for compositing.
#[derive(Debug, Clone)]
pub struct RenderedNotif {
    pub row: u16,
    pub col: u16,
    pub lines: Vec<String>,
}

/// Render a notification into positioned lines.
pub fn render_notification(
    text: &str,
    position: NotifPosition,
    term_w: u16,
    term_h: u16,
    max_width: usize,
) -> RenderedNotif {
    let wrapped = wrap_text(text, max_width);
    let notif_w = wrapped.iter().map(|l| l.len()).max().unwrap_or(0) as u16;
    let notif_h = wrapped.len() as u16;

    let col = match position {
        NotifPosition::TopRight | NotifPosition::BottomRight => term_w.saturating_sub(notif_w + 1),
        NotifPosition::TopCenter | NotifPosition::BottomCenter => {
            term_w.saturating_sub(notif_w) / 2
        }
    };
    let row = match position {
        NotifPosition::TopRight | NotifPosition::TopCenter => 1,
        NotifPosition::BottomRight | NotifPosition::BottomCenter => {
            term_h.saturating_sub(notif_h + 2)
        }
    };

    // Pad each line to the max width for a clean box
    let lines = wrapped
        .into_iter()
        .map(|l| {
            let mut s = l;
            if (s.len() as u16) < notif_w {
                s.extend(std::iter::repeat_n(' ', notif_w as usize - s.len()));
            }
            s
        })
        .collect();

    RenderedNotif { row, col, lines }
}

/// Wrap text into lines of at most `max_width` characters.
pub fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 {
        return vec![text.to_string()];
    }
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let chars: Vec<char> = paragraph.chars().collect();
        if chars.is_empty() {
            lines.push(String::new());
            continue;
        }
        for chunk in chars.chunks(max_width) {
            lines.push(chunk.iter().collect());
        }
    }
    lines
}

/// Maximum number of notifications visible on screen.
pub fn max_visible_notifications(term_height: u16) -> usize {
    // Reserve 4 rows for editor UI; each notif ~3 rows
    (term_height.saturating_sub(4) / 3) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_short() {
        let lines = wrap_text("hello", 10);
        assert_eq!(lines, vec!["hello"]);
    }

    #[test]
    fn wrap_long() {
        let lines = wrap_text("abcdefghij", 5);
        assert_eq!(lines, vec!["abcde", "fghij"]);
    }

    #[test]
    fn wrap_newline() {
        let lines = wrap_text("abc\ndef", 10);
        assert_eq!(lines, vec!["abc", "def"]);
    }

    #[test]
    fn render_top_right() {
        let n = render_notification("hi", NotifPosition::TopRight, 80, 24, 20);
        assert_eq!(n.row, 1);
        assert!(n.col > 50);
    }

    #[test]
    fn render_bottom_center() {
        let n = render_notification("ok", NotifPosition::BottomCenter, 80, 24, 20);
        assert!(n.row > 10);
        assert!(n.col > 30 && n.col < 50);
    }

    #[test]
    fn max_visible() {
        assert!(max_visible_notifications(24) >= 1);
        assert_eq!(max_visible_notifications(4), 0);
    }
}
