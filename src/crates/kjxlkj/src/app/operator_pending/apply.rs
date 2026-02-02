use kjxlkj_core::Buffer;
use kjxlkj_core_types::intent::Operator;

use super::{Motion, OperatorMotion};

/// Applies the completed operator motion to the buffer.
pub fn apply_operator_motion(buffer: &mut Buffer, op_motion: &OperatorMotion) {
    let count = op_motion.count.unwrap_or(1);

    match op_motion.motion {
        Motion::WholeLine => {
            // Delete/yank/change whole lines
            for _ in 0..count {
                delete_current_line(buffer);
            }
        }
        Motion::Left | Motion::Right => {
            let chars = count;
            match op_motion.operator {
                Operator::Delete | Operator::Change => {
                    for _ in 0..chars {
                        buffer.delete_char_at();
                    }
                }
                _ => {}
            }
        }
        Motion::Word | Motion::WordEnd => {
            // Simplified: delete to end of word
            if matches!(op_motion.operator, Operator::Delete | Operator::Change) {
                delete_to_word_end(buffer);
            }
        }
        Motion::LineEnd => {
            // D or d$
            if matches!(op_motion.operator, Operator::Delete | Operator::Change) {
                delete_to_line_end(buffer);
            }
        }
        _ => {}
    }
}

fn delete_current_line(buffer: &mut Buffer) {
    let line = buffer.cursor_line();
    let line_count = buffer.line_count();
    if line_count <= 1 {
        // Clear the only line
        buffer.set_line(line, "");
    } else {
        buffer.delete_line(line);
    }
}

fn delete_to_word_end(buffer: &mut Buffer) {
    // Simplified word deletion
    while let Some(c) = buffer.current_char() {
        if c.is_whitespace() {
            break;
        }
        buffer.delete_char_at();
    }
}

fn delete_to_line_end(buffer: &mut Buffer) {
    while let Some(c) = buffer.current_char() {
        if c == '\n' {
            break;
        }
        buffer.delete_char_at();
    }
}
