//! Message and command-line area rendering.

use serde::{Deserialize, Serialize};

/// Kind of message displayed in the message area.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageKind {
    Info,
    Error,
    Warning,
}

/// State of the message area widget.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageArea {
    pub message: String,
    pub kind: MessageKind,
    pub visible: bool,
}

impl MessageArea {
    /// Create a new visible message area.
    pub fn new(message: impl Into<String>, kind: MessageKind) -> Self {
        Self { message: message.into(), kind, visible: true }
    }

    /// Dismiss the current message.
    pub fn dismiss(&mut self) {
        self.visible = false;
        self.message.clear();
    }
}

/// Render the message area to a fixed-width string.
pub fn render_message_area(message: &str, kind: MessageKind, width: usize) -> String {
    let prefix = match kind {
        MessageKind::Error => "E: ",
        MessageKind::Warning => "W: ",
        MessageKind::Info => "",
    };
    let full = format!("{prefix}{message}");
    if full.len() >= width {
        full[..width].to_string()
    } else {
        let mut out = full;
        out.extend(std::iter::repeat(' ').take(width - out.len()));
        out
    }
}

/// Format an error message with the standard `E: ` prefix.
pub fn format_error(msg: &str) -> String {
    format!("E: {msg}")
}

/// Format an informational message (no prefix).
pub fn format_info(msg: &str) -> String {
    msg.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_info() {
        let s = render_message_area("hello", MessageKind::Info, 10);
        assert_eq!(s.len(), 10);
        assert!(s.starts_with("hello"));
    }

    #[test]
    fn render_error() {
        let s = render_message_area("fail", MessageKind::Error, 20);
        assert!(s.starts_with("E: fail"));
    }

    #[test]
    fn render_warning() {
        let s = render_message_area("warn", MessageKind::Warning, 20);
        assert!(s.starts_with("W: warn"));
    }

    #[test]
    fn format_helpers() {
        assert_eq!(format_error("bad"), "E: bad");
        assert_eq!(format_info("ok"), "ok");
    }

    #[test]
    fn truncation() {
        let s = render_message_area("a very long message", MessageKind::Info, 5);
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn message_area_dismiss() {
        let mut ma = MessageArea::new("hello", MessageKind::Info);
        assert!(ma.visible);
        ma.dismiss();
        assert!(!ma.visible);
        assert!(ma.message.is_empty());
    }
}
