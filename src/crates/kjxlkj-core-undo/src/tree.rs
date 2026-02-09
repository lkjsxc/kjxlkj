use kjxlkj_core_types::BufferVersion;

/// A single undo entry representing one group of edits.
#[derive(Debug, Clone)]
pub struct UndoEntry {
    /// Buffer version before this edit group.
    pub version_before: BufferVersion,
    /// Rope snapshot before edits (for undo).
    pub content_before: ropey::Rope,
    /// Cursor position before edits.
    pub cursor_line: usize,
    pub cursor_grapheme: usize,
}

/// Undo tree with linear history and branches.
///
/// Currently implemented as a linear undo stack with
/// a redo stack. Future: full tree with branch navigation.
#[derive(Debug)]
pub struct UndoTree {
    undo_stack: Vec<UndoEntry>,
    redo_stack: Vec<UndoEntry>,
    /// Whether we are currently in an open undo group.
    group_open: bool,
}

impl UndoTree {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            group_open: false,
        }
    }

    /// Begin an undo group (e.g., on entering insert mode).
    pub fn begin_group(
        &mut self,
        version: BufferVersion,
        content: ropey::Rope,
        cursor_line: usize,
        cursor_grapheme: usize,
    ) {
        if !self.group_open {
            self.undo_stack.push(UndoEntry {
                version_before: version,
                content_before: content,
                cursor_line,
                cursor_grapheme,
            });
            self.redo_stack.clear();
            self.group_open = true;
        }
    }

    /// End the current undo group.
    pub fn end_group(&mut self) {
        self.group_open = false;
    }

    /// Push a single-edit undo entry (for normal mode changes).
    pub fn push(
        &mut self,
        version: BufferVersion,
        content: ropey::Rope,
        cursor_line: usize,
        cursor_grapheme: usize,
    ) {
        self.undo_stack.push(UndoEntry {
            version_before: version,
            content_before: content,
            cursor_line,
            cursor_grapheme,
        });
        self.redo_stack.clear();
    }

    /// Undo: pop from undo stack, push current state to redo.
    ///
    /// Returns the entry to restore, or None if nothing to undo.
    pub fn undo(
        &mut self,
        current_version: BufferVersion,
        current_content: ropey::Rope,
        cursor_line: usize,
        cursor_grapheme: usize,
    ) -> Option<&UndoEntry> {
        if self.undo_stack.last().is_some() {
            self.redo_stack.push(UndoEntry {
                version_before: current_version,
                content_before: current_content,
                cursor_line,
                cursor_grapheme,
            });
            // Return the last undo entry (not popped yet for borrow)
        }
        self.undo_stack.last()
    }

    /// Pop the last undo entry (call after using the reference).
    pub fn pop_undo(&mut self) -> Option<UndoEntry> {
        self.undo_stack.pop()
    }

    /// Redo: pop from redo stack.
    pub fn redo(
        &mut self,
        current_version: BufferVersion,
        current_content: ropey::Rope,
        cursor_line: usize,
        cursor_grapheme: usize,
    ) -> Option<UndoEntry> {
        if let Some(entry) = self.redo_stack.pop() {
            self.undo_stack.push(UndoEntry {
                version_before: current_version,
                content_before: current_content,
                cursor_line,
                cursor_grapheme,
            });
            Some(entry)
        } else {
            None
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}

impl Default for UndoTree {
    fn default() -> Self {
        Self::new()
    }
}
