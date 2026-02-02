//! Edit operation executor.

use crate::operation::{EditOperation, OperationResult};
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::position::Position;

/// Executes edit operations on a buffer.
pub struct EditExecutor;

impl EditExecutor {
    /// Executes an operation on a buffer.
    pub fn execute(buffer: &mut TextBuffer, operation: &EditOperation) -> Option<OperationResult> {
        match operation {
            EditOperation::Insert { position, text } => {
                Self::execute_insert(buffer, *position, text)
            }
            EditOperation::Delete { start, end, .. } => Self::execute_delete(buffer, *start, *end),
            EditOperation::Replace {
                start,
                end,
                new_text,
                ..
            } => Self::execute_replace(buffer, *start, *end, new_text),
            EditOperation::Batch(ops) => {
                let mut last_result = None;
                let mut undo_ops = Vec::new();
                for op in ops {
                    if let Some(result) = Self::execute(buffer, op) {
                        undo_ops.push(result.undo);
                        last_result = Some(result.cursor);
                    }
                }
                last_result.map(|cursor| OperationResult {
                    cursor,
                    undo: EditOperation::Batch(undo_ops.into_iter().rev().collect()),
                })
            }
        }
    }

    fn execute_insert(
        buffer: &mut TextBuffer,
        position: Position,
        text: &str,
    ) -> Option<OperationResult> {
        let idx = buffer.pos_to_idx(position)?;
        buffer.rope_mut().insert(idx, text);

        // Calculate new cursor position
        let new_idx = idx + text.chars().count();
        let new_pos = buffer.idx_to_pos(new_idx).unwrap_or(position);

        Some(OperationResult {
            cursor: new_pos,
            undo: EditOperation::delete(position, new_pos, text.to_string()),
        })
    }

    fn execute_delete(
        buffer: &mut TextBuffer,
        start: Position,
        end: Position,
    ) -> Option<OperationResult> {
        let start_idx = buffer.pos_to_idx(start)?;
        let end_idx = buffer.pos_to_idx(end)?;

        let deleted = buffer.rope().slice(start_idx, end_idx);
        buffer.rope_mut().remove(start_idx, end_idx);

        Some(OperationResult {
            cursor: start,
            undo: EditOperation::insert(start, deleted),
        })
    }

    fn execute_replace(
        buffer: &mut TextBuffer,
        start: Position,
        end: Position,
        new_text: &str,
    ) -> Option<OperationResult> {
        let start_idx = buffer.pos_to_idx(start)?;
        let end_idx = buffer.pos_to_idx(end)?;

        let old_text = buffer.rope().slice(start_idx, end_idx);
        buffer.rope_mut().replace(start_idx, end_idx, new_text);

        let new_idx = start_idx + new_text.chars().count();
        let new_pos = buffer.idx_to_pos(new_idx).unwrap_or(start);

        Some(OperationResult {
            cursor: new_pos,
            undo: EditOperation::Replace {
                start,
                end: new_pos,
                old_text: new_text.to_string(),
                new_text: old_text,
            },
        })
    }
}
