//! Dot-repeat functionality for repeating last change.

use kjxlkj_core_types::motion::Motion;
use kjxlkj_core_types::operator::Operator;
use kjxlkj_core_types::text_object::TextObject;

/// A recorded change that can be repeated.
#[derive(Debug, Clone)]
pub enum RecordedChange {
    /// Operator with motion (e.g., dw, cw, y$).
    OperatorMotion {
        operator: Operator,
        motion: Motion,
        count: usize,
    },
    /// Operator with text object (e.g., diw, ci").
    OperatorTextObject {
        operator: Operator,
        text_object: TextObject,
        count: usize,
    },
    /// Line operator (e.g., dd, cc, yy).
    LineOperator { operator: Operator, count: usize },
    /// Insert with text (e.g., i, a, o).
    InsertText {
        text: String,
        insert_type: InsertType,
    },
    /// Replace character (r).
    ReplaceChar { char: char, count: usize },
    /// Delete character (x).
    DeleteChar { count: usize },
}

/// Type of insert operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertType {
    /// Insert before cursor (i).
    Before,
    /// Insert after cursor (a).
    After,
    /// Insert at start of line (I).
    LineStart,
    /// Insert at end of line (A).
    LineEnd,
    /// Open line below (o).
    OpenBelow,
    /// Open line above (O).
    OpenAbove,
}

/// Tracks the last change for dot-repeat.
#[derive(Debug, Clone, Default)]
pub struct RepeatTracker {
    /// The last recorded change.
    last_change: Option<RecordedChange>,
    /// Text accumulated during insert mode.
    insert_buffer: String,
    /// Type of current insert.
    current_insert: Option<InsertType>,
}

impl RepeatTracker {
    /// Creates a new repeat tracker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records an operator with motion.
    pub fn record_operator_motion(&mut self, op: Operator, motion: Motion, count: usize) {
        self.last_change = Some(RecordedChange::OperatorMotion {
            operator: op,
            motion,
            count,
        });
    }

    /// Records an operator with text object.
    pub fn record_operator_text_object(&mut self, op: Operator, obj: TextObject, count: usize) {
        self.last_change = Some(RecordedChange::OperatorTextObject {
            operator: op,
            text_object: obj,
            count,
        });
    }

    /// Records a line operator.
    pub fn record_line_operator(&mut self, op: Operator, count: usize) {
        self.last_change = Some(RecordedChange::LineOperator {
            operator: op,
            count,
        });
    }

    /// Starts recording an insert.
    pub fn start_insert(&mut self, insert_type: InsertType) {
        self.insert_buffer.clear();
        self.current_insert = Some(insert_type);
    }

    /// Adds a character to the insert buffer.
    pub fn record_insert_char(&mut self, ch: char) {
        self.insert_buffer.push(ch);
    }

    /// Records a backspace in insert.
    pub fn record_insert_backspace(&mut self) {
        self.insert_buffer.pop();
    }

    /// Finishes recording an insert.
    pub fn finish_insert(&mut self) {
        if let Some(insert_type) = self.current_insert.take() {
            self.last_change = Some(RecordedChange::InsertText {
                text: std::mem::take(&mut self.insert_buffer),
                insert_type,
            });
        }
    }

    /// Records a character replacement.
    pub fn record_replace_char(&mut self, ch: char, count: usize) {
        self.last_change = Some(RecordedChange::ReplaceChar { char: ch, count });
    }

    /// Records character deletion.
    pub fn record_delete_char(&mut self, count: usize) {
        self.last_change = Some(RecordedChange::DeleteChar { count });
    }

    /// Gets the last recorded change.
    pub fn last_change(&self) -> Option<&RecordedChange> {
        self.last_change.as_ref()
    }

    /// Clears the last change.
    pub fn clear(&mut self) {
        self.last_change = None;
        self.insert_buffer.clear();
        self.current_insert = None;
    }
}
