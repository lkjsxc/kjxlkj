//! Editing primitives — operations, motions, text objects.

use kjxlkj_core_types::{Position, Range};

/// A single atomic edit operation on a buffer.
#[derive(Debug, Clone)]
pub enum EditOp {
    /// Insert text at a position.
    Insert { pos: Position, text: String },
    /// Delete text in a range.
    Delete { range: Range },
    /// Replace text in a range with new text.
    Replace { range: Range, text: String },
}

/// A batch of edit operations applied together as a single logical change.
#[derive(Debug, Clone, Default)]
pub struct EditTransaction {
    ops: Vec<EditOp>,
}

impl EditTransaction {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, op: EditOp) {
        self.ops.push(op);
    }

    pub fn ops(&self) -> &[EditOp] {
        &self.ops
    }

    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }
}

/// Describes a cursor motion direction and distance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motion {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
    LineStart,
    LineEnd,
    FirstNonBlank,
    WordForward(usize),
    WordBackward(usize),
    WordEndForward(usize),
    BigWordForward(usize),
    BigWordBackward(usize),
    BigWordEndForward(usize),
    BufferTop,
    BufferBottom,
    GotoLine(usize),
    MatchParen,
}

/// Describes a Vim-style operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Delete,
    Yank,
    Change,
    Indent,
    Outdent,
    AutoIndent,
    ToUpperCase,
    ToLowerCase,
    SwapCase,
}

/// Describes a text object (inner or around).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextObject {
    InnerWord,
    AWord,
    InnerBigWord,
    ABigWord,
    InnerParagraph,
    AParagraph,
    InnerSentence,
    ASentence,
    InnerParen,
    AParen,
    InnerBracket,
    ABracket,
    InnerBrace,
    ABrace,
    InnerAngle,
    AAngle,
    InnerDoubleQuote,
    ADoubleQuote,
    InnerSingleQuote,
    ASingleQuote,
    InnerBacktick,
    ABacktick,
    InnerTag,
    ATag,
}
