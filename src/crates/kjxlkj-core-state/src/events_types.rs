//! Event type definitions for the autocmd system.

/// Event types that the editor can fire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventKind {
    // Buffer events
    BufferNew,
    BufferRead,
    BufferWritePre,
    BufferWrite,
    BufferWritePost,
    BufferEnter,
    BufferLeave,
    BufferDelete,
    BufferModified,
    BufferUnmodified,
    // Window events
    WindowNew,
    WindowClosed,
    WindowEnter,
    WindowLeave,
    WindowResize,
    // Mode events
    ModeChanged,
    InsertEnter,
    InsertLeave,
    VisualEnter,
    VisualLeave,
    // Cursor events
    CursorMoved,
    CursorHold,
    CursorHoldInsert,
    // File events
    FileType,
    FileChanged,
    FileReadCmd,
    // Application events
    AppEnter,
    AppLeave,
    AppSuspend,
    AppResume,
    ExitPre,
}

/// Data passed to event handlers.
#[derive(Debug, Clone, Default)]
pub struct EventData {
    /// Buffer ID, if applicable.
    pub buffer: Option<usize>,
    /// File path, if applicable.
    pub file: Option<String>,
    /// Old mode string, for ModeChanged.
    pub old_mode: Option<String>,
    /// New mode string, for ModeChanged.
    pub new_mode: Option<String>,
    /// Matched pattern, for pattern-filtered events.
    pub matched: Option<String>,
}

/// A registered event handler.
#[derive(Debug, Clone)]
pub struct EventHandler {
    /// Unique handler ID.
    pub id: u64,
    /// Event this handler responds to.
    pub event: EventKind,
    /// Optional file pattern filter (glob).
    pub pattern: Option<String>,
    /// Command string to execute when event fires.
    pub command: String,
    /// Optional group name.
    pub group: Option<String>,
    /// Whether handler has been disabled.
    pub enabled: bool,
}
