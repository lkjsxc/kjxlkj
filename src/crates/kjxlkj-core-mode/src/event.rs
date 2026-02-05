//! Event automation system for kjxlkj editor.
//!
//! Implements event-driven automation as specified in
//! `/docs/spec/scripting/event-automation.md`.
//!
//! This module provides:
//! - Event type definitions
//! - Event handler registration and dispatch
//! - Event groups for organized hook management

use std::collections::HashMap;

/// Event types that can be hooked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventType {
    // Buffer events
    /// Buffer created.
    BufferNew,
    /// File read into buffer.
    BufferRead,
    /// Buffer written to file.
    BufferWrite,
    /// Before buffer write.
    BufferWritePre,
    /// After buffer write.
    BufferWritePost,
    /// Entered buffer.
    BufferEnter,
    /// Left buffer.
    BufferLeave,
    /// Buffer deleted.
    BufferDelete,
    /// Content changed.
    BufferModified,
    /// Saved (unmodified).
    BufferUnmodified,

    // Window events
    /// Window created.
    WindowNew,
    /// Window closed.
    WindowClosed,
    /// Entered window.
    WindowEnter,
    /// Left window.
    WindowLeave,
    /// Window resized.
    WindowResize,

    // Mode events
    /// Mode transition.
    ModeChanged,
    /// Entered insert mode.
    InsertEnter,
    /// Left insert mode.
    InsertLeave,
    /// Entered visual mode.
    VisualEnter,
    /// Left visual mode.
    VisualLeave,

    // Cursor events
    /// Cursor position changed.
    CursorMoved,
    /// Cursor idle timeout.
    CursorHold,
    /// Idle in insert mode.
    CursorHoldInsert,

    // File events
    /// Filetype detected.
    FileType,
    /// External file change.
    FileChanged,
    /// Read command file.
    FileReadCmd,

    // Application events
    /// Editor gained focus.
    AppEnter,
    /// Editor lost focus.
    AppLeave,
    /// Editor suspended.
    AppSuspend,
    /// Editor resumed.
    AppResume,
    /// Before exit.
    ExitPre,
}

impl EventType {
    /// Get the string name for this event type.
    pub fn name(&self) -> &'static str {
        match self {
            Self::BufferNew => "buffer_new",
            Self::BufferRead => "buffer_read",
            Self::BufferWrite => "buffer_write",
            Self::BufferWritePre => "buffer_write_pre",
            Self::BufferWritePost => "buffer_write_post",
            Self::BufferEnter => "buffer_enter",
            Self::BufferLeave => "buffer_leave",
            Self::BufferDelete => "buffer_delete",
            Self::BufferModified => "buffer_modified",
            Self::BufferUnmodified => "buffer_unmodified",
            Self::WindowNew => "window_new",
            Self::WindowClosed => "window_closed",
            Self::WindowEnter => "window_enter",
            Self::WindowLeave => "window_leave",
            Self::WindowResize => "window_resize",
            Self::ModeChanged => "mode_changed",
            Self::InsertEnter => "insert_enter",
            Self::InsertLeave => "insert_leave",
            Self::VisualEnter => "visual_enter",
            Self::VisualLeave => "visual_leave",
            Self::CursorMoved => "cursor_moved",
            Self::CursorHold => "cursor_hold",
            Self::CursorHoldInsert => "cursor_hold_insert",
            Self::FileType => "file_type",
            Self::FileChanged => "file_changed",
            Self::FileReadCmd => "file_read_cmd",
            Self::AppEnter => "app_enter",
            Self::AppLeave => "app_leave",
            Self::AppSuspend => "app_suspend",
            Self::AppResume => "app_resume",
            Self::ExitPre => "exit_pre",
        }
    }

    /// Parse an event type from its string name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "buffer_new" => Some(Self::BufferNew),
            "buffer_read" => Some(Self::BufferRead),
            "buffer_write" => Some(Self::BufferWrite),
            "buffer_write_pre" => Some(Self::BufferWritePre),
            "buffer_write_post" => Some(Self::BufferWritePost),
            "buffer_enter" => Some(Self::BufferEnter),
            "buffer_leave" => Some(Self::BufferLeave),
            "buffer_delete" => Some(Self::BufferDelete),
            "buffer_modified" => Some(Self::BufferModified),
            "buffer_unmodified" => Some(Self::BufferUnmodified),
            "window_new" => Some(Self::WindowNew),
            "window_closed" => Some(Self::WindowClosed),
            "window_enter" => Some(Self::WindowEnter),
            "window_leave" => Some(Self::WindowLeave),
            "window_resize" => Some(Self::WindowResize),
            "mode_changed" => Some(Self::ModeChanged),
            "insert_enter" => Some(Self::InsertEnter),
            "insert_leave" => Some(Self::InsertLeave),
            "visual_enter" => Some(Self::VisualEnter),
            "visual_leave" => Some(Self::VisualLeave),
            "cursor_moved" => Some(Self::CursorMoved),
            "cursor_hold" => Some(Self::CursorHold),
            "cursor_hold_insert" => Some(Self::CursorHoldInsert),
            "file_type" => Some(Self::FileType),
            "file_changed" => Some(Self::FileChanged),
            "file_read_cmd" => Some(Self::FileReadCmd),
            "app_enter" => Some(Self::AppEnter),
            "app_leave" => Some(Self::AppLeave),
            "app_suspend" => Some(Self::AppSuspend),
            "app_resume" => Some(Self::AppResume),
            "exit_pre" => Some(Self::ExitPre),
            _ => None,
        }
    }
}

/// Data associated with an event.
#[derive(Debug, Clone, Default)]
pub struct EventData {
    /// Buffer ID if relevant.
    pub buffer_id: Option<u64>,
    /// File path if relevant.
    pub file_path: Option<String>,
    /// Filetype if relevant.
    pub filetype: Option<String>,
    /// Mode name if relevant.
    pub mode: Option<String>,
    /// Cursor line.
    pub line: Option<usize>,
    /// Cursor column.
    pub column: Option<usize>,
    /// Additional context.
    pub extra: HashMap<String, String>,
}

impl EventData {
    /// Create empty event data.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create event data for a buffer.
    pub fn for_buffer(buffer_id: u64) -> Self {
        Self {
            buffer_id: Some(buffer_id),
            ..Default::default()
        }
    }

    /// Create event data for a file.
    pub fn for_file(path: impl Into<String>) -> Self {
        Self {
            file_path: Some(path.into()),
            ..Default::default()
        }
    }

    /// Create event data for cursor position.
    pub fn for_cursor(line: usize, column: usize) -> Self {
        Self {
            line: Some(line),
            column: Some(column),
            ..Default::default()
        }
    }

    /// Set the mode.
    pub fn with_mode(mut self, mode: impl Into<String>) -> Self {
        self.mode = Some(mode.into());
        self
    }

    /// Add extra context.
    pub fn with_extra(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }
}

/// A registered event handler.
#[derive(Debug, Clone)]
pub struct EventHandler {
    /// Handler ID.
    pub id: u64,
    /// Event type to handle.
    pub event: EventType,
    /// Command to execute.
    pub command: String,
    /// Pattern to match (for file patterns).
    pub pattern: Option<String>,
    /// Group this handler belongs to.
    pub group: Option<String>,
    /// Whether this handler is enabled.
    pub enabled: bool,
}

impl EventHandler {
    /// Create a new event handler.
    pub fn new(id: u64, event: EventType, command: impl Into<String>) -> Self {
        Self {
            id,
            event,
            command: command.into(),
            pattern: None,
            group: None,
            enabled: true,
        }
    }

    /// Set a file pattern for this handler.
    pub fn with_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = Some(pattern.into());
        self
    }

    /// Set the group for this handler.
    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Check if this handler matches the given event and data.
    pub fn matches(&self, event: EventType, data: &EventData) -> bool {
        if !self.enabled || self.event != event {
            return false;
        }

        // Check pattern match if specified
        if let Some(pattern) = &self.pattern {
            if let Some(path) = &data.file_path {
                // Simple glob-like matching
                if !simple_pattern_match(pattern, path) {
                    return false;
                }
            }
        }

        true
    }
}

/// Simple glob pattern matching.
fn simple_pattern_match(pattern: &str, text: &str) -> bool {
    if pattern == "*" {
        return true;
    }

    // Handle *.ext patterns
    if let Some(ext) = pattern.strip_prefix("*.") {
        return text.ends_with(&format!(".{}", ext));
    }

    // Handle prefix* patterns
    if let Some(prefix) = pattern.strip_suffix('*') {
        return text.starts_with(prefix);
    }

    // Exact match
    pattern == text
}

/// Registry for event handlers.
#[derive(Debug, Default)]
pub struct EventRegistry {
    /// All registered handlers.
    handlers: Vec<EventHandler>,
    /// Next handler ID.
    next_id: u64,
}

impl EventRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new event handler.
    pub fn register(&mut self, event: EventType, command: impl Into<String>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.handlers.push(EventHandler::new(id, event, command));
        id
    }

    /// Register a handler with options.
    pub fn register_handler(&mut self, handler: EventHandler) -> u64 {
        let id = handler.id.max(self.next_id);
        self.next_id = id + 1;
        self.handlers.push(handler);
        id
    }

    /// Unregister a handler by ID.
    pub fn unregister(&mut self, id: u64) -> bool {
        if let Some(pos) = self.handlers.iter().position(|h| h.id == id) {
            self.handlers.remove(pos);
            true
        } else {
            false
        }
    }

    /// Clear all handlers for an event type.
    pub fn clear_event(&mut self, event: EventType) {
        self.handlers.retain(|h| h.event != event);
    }

    /// Clear all handlers in a group.
    pub fn clear_group(&mut self, group: &str) {
        self.handlers.retain(|h| h.group.as_deref() != Some(group));
    }

    /// Get all matching handlers for an event.
    pub fn get_handlers(&self, event: EventType, data: &EventData) -> Vec<&EventHandler> {
        self.handlers
            .iter()
            .filter(|h| h.matches(event, data))
            .collect()
    }

    /// Get commands to execute for an event.
    pub fn get_commands(&self, event: EventType, data: &EventData) -> Vec<&str> {
        self.get_handlers(event, data)
            .iter()
            .map(|h| h.command.as_str())
            .collect()
    }

    /// Get the number of registered handlers.
    pub fn len(&self) -> usize {
        self.handlers.len()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.handlers.is_empty()
    }

    /// List all handlers for an event type.
    pub fn list(&self, event: EventType) -> Vec<&EventHandler> {
        self.handlers.iter().filter(|h| h.event == event).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_type_name() {
        assert_eq!(EventType::BufferNew.name(), "buffer_new");
        assert_eq!(EventType::InsertEnter.name(), "insert_enter");
        assert_eq!(EventType::ExitPre.name(), "exit_pre");
    }

    #[test]
    fn test_event_type_from_name() {
        assert_eq!(
            EventType::from_name("buffer_new"),
            Some(EventType::BufferNew)
        );
        assert_eq!(
            EventType::from_name("insert_enter"),
            Some(EventType::InsertEnter)
        );
        assert_eq!(EventType::from_name("invalid"), None);
    }

    #[test]
    fn test_event_type_roundtrip() {
        let event = EventType::CursorMoved;
        let name = event.name();
        assert_eq!(EventType::from_name(name), Some(event));
    }

    #[test]
    fn test_event_data_new() {
        let data = EventData::new();
        assert!(data.buffer_id.is_none());
        assert!(data.file_path.is_none());
    }

    #[test]
    fn test_event_data_for_buffer() {
        let data = EventData::for_buffer(42);
        assert_eq!(data.buffer_id, Some(42));
    }

    #[test]
    fn test_event_data_for_file() {
        let data = EventData::for_file("test.rs");
        assert_eq!(data.file_path, Some("test.rs".to_string()));
    }

    #[test]
    fn test_event_data_for_cursor() {
        let data = EventData::for_cursor(10, 5);
        assert_eq!(data.line, Some(10));
        assert_eq!(data.column, Some(5));
    }

    #[test]
    fn test_event_data_with_mode() {
        let data = EventData::new().with_mode("Normal");
        assert_eq!(data.mode, Some("Normal".to_string()));
    }

    #[test]
    fn test_event_data_with_extra() {
        let data = EventData::new().with_extra("key", "value");
        assert_eq!(data.extra.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_event_handler_new() {
        let handler = EventHandler::new(1, EventType::BufferNew, "echo hi");
        assert_eq!(handler.id, 1);
        assert_eq!(handler.event, EventType::BufferNew);
        assert_eq!(handler.command, "echo hi");
        assert!(handler.enabled);
    }

    #[test]
    fn test_event_handler_with_pattern() {
        let handler = EventHandler::new(1, EventType::BufferNew, "cmd")
            .with_pattern("*.rs");
        assert_eq!(handler.pattern, Some("*.rs".to_string()));
    }

    #[test]
    fn test_event_handler_with_group() {
        let handler = EventHandler::new(1, EventType::BufferNew, "cmd")
            .with_group("mygroup");
        assert_eq!(handler.group, Some("mygroup".to_string()));
    }

    #[test]
    fn test_event_handler_matches_basic() {
        let handler = EventHandler::new(1, EventType::BufferNew, "cmd");
        let data = EventData::new();

        assert!(handler.matches(EventType::BufferNew, &data));
        assert!(!handler.matches(EventType::BufferLeave, &data));
    }

    #[test]
    fn test_event_handler_matches_pattern() {
        let handler = EventHandler::new(1, EventType::FileType, "cmd")
            .with_pattern("*.rs");

        let data_match = EventData::for_file("test.rs");
        let data_no_match = EventData::for_file("test.py");

        assert!(handler.matches(EventType::FileType, &data_match));
        assert!(!handler.matches(EventType::FileType, &data_no_match));
    }

    #[test]
    fn test_simple_pattern_match_wildcard() {
        assert!(simple_pattern_match("*", "anything"));
    }

    #[test]
    fn test_simple_pattern_match_extension() {
        assert!(simple_pattern_match("*.rs", "test.rs"));
        assert!(!simple_pattern_match("*.rs", "test.py"));
    }

    #[test]
    fn test_simple_pattern_match_prefix() {
        assert!(simple_pattern_match("test*", "testing"));
        assert!(!simple_pattern_match("test*", "other"));
    }

    #[test]
    fn test_registry_register() {
        let mut registry = EventRegistry::new();
        let id = registry.register(EventType::BufferNew, "echo hi");
        assert_eq!(id, 0);
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn test_registry_unregister() {
        let mut registry = EventRegistry::new();
        let id = registry.register(EventType::BufferNew, "cmd");
        assert!(registry.unregister(id));
        assert!(registry.is_empty());
    }

    #[test]
    fn test_registry_clear_event() {
        let mut registry = EventRegistry::new();
        registry.register(EventType::BufferNew, "cmd1");
        registry.register(EventType::BufferNew, "cmd2");
        registry.register(EventType::BufferLeave, "cmd3");

        registry.clear_event(EventType::BufferNew);

        assert_eq!(registry.len(), 1);
        assert_eq!(registry.list(EventType::BufferLeave).len(), 1);
    }

    #[test]
    fn test_registry_clear_group() {
        let mut registry = EventRegistry::new();
        let h1 = EventHandler::new(0, EventType::BufferNew, "cmd1").with_group("g1");
        let h2 = EventHandler::new(1, EventType::BufferNew, "cmd2").with_group("g2");
        registry.register_handler(h1);
        registry.register_handler(h2);

        registry.clear_group("g1");

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn test_registry_get_commands() {
        let mut registry = EventRegistry::new();
        registry.register(EventType::BufferNew, "echo 1");
        registry.register(EventType::BufferNew, "echo 2");

        let data = EventData::new();
        let cmds = registry.get_commands(EventType::BufferNew, &data);

        assert_eq!(cmds.len(), 2);
        assert!(cmds.contains(&"echo 1"));
        assert!(cmds.contains(&"echo 2"));
    }

    #[test]
    fn test_registry_list() {
        let mut registry = EventRegistry::new();
        registry.register(EventType::BufferNew, "cmd1");
        registry.register(EventType::InsertEnter, "cmd2");

        let list = registry.list(EventType::BufferNew);
        assert_eq!(list.len(), 1);
    }
}
