//! Edit operators.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{
    Cursor, EditorResult, Position, Range, RegisterContent, RegisterType,
};

/// Delete the character under the cursor.
pub fn delete_char(buffer: &mut TextBuffer, cursor: &Cursor) -> EditorResult<String> {
    let line_len = buffer.line_len(cursor.line() as usize);
    if line_len == 0 {
        return Ok(String::new());
    }

    let start = cursor.position;
    let end = Position::new(cursor.line(), cursor.col() + 1);
    buffer.delete(Range::new(start, end))
}

/// Delete current line.
pub fn delete_line(buffer: &mut TextBuffer, cursor: &Cursor) -> EditorResult<RegisterContent> {
    let deleted = buffer.delete_line(cursor.line() as usize)?;
    Ok(RegisterContent::line(deleted))
}

/// Yank (copy) current line.
pub fn yank_line(buffer: &TextBuffer, cursor: &Cursor) -> Option<RegisterContent> {
    buffer
        .line(cursor.line() as usize)
        .map(|line| RegisterContent::new(format!("{}\n", line), RegisterType::Line))
}

/// Delete backward (backspace).
pub fn delete_backward(buffer: &mut TextBuffer, cursor: &mut Cursor) -> EditorResult<()> {
    if cursor.col() > 0 {
        let start = Position::new(cursor.line(), cursor.col() - 1);
        let end = cursor.position;
        buffer.delete(Range::new(start, end))?;
        cursor.move_horizontal(cursor.col() - 1);
    } else if cursor.line() > 0 {
        // Join with previous line
        let prev_line = cursor.line() - 1;
        let prev_len = buffer.line_len(prev_line as usize);

        // Delete the newline at end of previous line
        let line_start = Position::new(cursor.line(), 0);
        let newline_pos = Position::new(prev_line, prev_len as u32);
        buffer.delete(Range::new(newline_pos, line_start))?;

        cursor.move_to(prev_line, prev_len as u32);
    }
    Ok(())
}

/// Delete forward (delete key).
pub fn delete_forward(buffer: &mut TextBuffer, cursor: &Cursor) -> EditorResult<()> {
    let line_len = buffer.line_len(cursor.line() as usize);
    if (cursor.col() as usize) < line_len {
        let start = cursor.position;
        let end = Position::new(cursor.line(), cursor.col() + 1);
        buffer.delete(Range::new(start, end))?;
    } else if (cursor.line() as usize) < buffer.line_count() - 1 {
        // Join with next line
        let start = Position::new(cursor.line(), line_len as u32);
        let end = Position::new(cursor.line() + 1, 0);
        buffer.delete(Range::new(start, end))?;
    }
    Ok(())
}

/// Insert text at cursor position.
pub fn insert_text(buffer: &mut TextBuffer, cursor: &mut Cursor, text: &str) -> EditorResult<()> {
    buffer.insert(cursor.position, text)?;
    let grapheme_count = kjxlkj_core_text::grapheme_count(text);
    let new_col = cursor.col() + grapheme_count as u32;
    cursor.move_horizontal(new_col);
    Ok(())
}

/// Insert a newline and handle indentation.
pub fn insert_newline(
    buffer: &mut TextBuffer,
    cursor: &mut Cursor,
    line_ending: &str,
) -> EditorResult<()> {
    // Get current line's indentation
    let indent = buffer
        .line(cursor.line() as usize)
        .map(|line| {
            let spaces: String = line.chars().take_while(|c| c.is_whitespace()).collect();
            spaces
        })
        .unwrap_or_default();

    let text = format!("{}{}", line_ending, indent);
    buffer.insert(cursor.position, &text)?;

    cursor.move_to(cursor.line() + 1, indent.len() as u32);
    Ok(())
}

/// Open a new line below and position cursor.
pub fn open_line_below(
    buffer: &mut TextBuffer,
    cursor: &mut Cursor,
    _line_ending: &str,
) -> EditorResult<()> {
    let current_line = cursor.line() as usize;
    let indent = buffer
        .line(current_line)
        .map(|line| {
            let spaces: String = line.chars().take_while(|c| c.is_whitespace()).collect();
            spaces
        })
        .unwrap_or_default();

    buffer.insert_line(current_line + 1, &indent)?;
    cursor.move_to(cursor.line() + 1, indent.len() as u32);
    Ok(())
}

/// Paste content after cursor.
pub fn paste_after(
    buffer: &mut TextBuffer,
    cursor: &mut Cursor,
    content: &RegisterContent,
) -> EditorResult<()> {
    match content.reg_type {
        RegisterType::Line => {
            // Paste on new line below
            buffer.insert_line(cursor.line() as usize + 1, &content.text)?;
            cursor.move_to(cursor.line() + 1, 0);
        }
        RegisterType::Char | RegisterType::Block => {
            // Paste after cursor position
            let pos = Position::new(cursor.line(), cursor.col() + 1);
            buffer.insert(pos, &content.text)?;
            cursor.move_horizontal(cursor.col() + 1);
        }
    }
    Ok(())
}

/// Paste content before cursor.
pub fn paste_before(
    buffer: &mut TextBuffer,
    cursor: &mut Cursor,
    content: &RegisterContent,
) -> EditorResult<()> {
    match content.reg_type {
        RegisterType::Line => {
            // Paste on line above
            buffer.insert_line(cursor.line() as usize, &content.text)?;
        }
        RegisterType::Char | RegisterType::Block => {
            // Paste at cursor position
            buffer.insert(cursor.position, &content.text)?;
        }
    }
    Ok(())
}
