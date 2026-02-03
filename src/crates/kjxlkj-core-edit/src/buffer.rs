//! Buffer with editing capabilities.

use kjxlkj_core_text::RopeText;
use kjxlkj_core_types::{BufferId, BufferVersion, Cursor, LineCol};
use kjxlkj_core_undo::{EditOperation, UndoGroup, UndoHistory};

/// A text buffer with editing support.
#[derive(Debug)]
pub struct Buffer {
    id: BufferId,
    name: String,
    path: Option<String>,
    text: RopeText,
    cursor: Cursor,
    history: UndoHistory,
    modified: bool,
    yank_register: String,
}

impl Buffer {
    /// Creates a new empty buffer.
    pub fn new(id: BufferId, name: String) -> Self {
        Self {
            id,
            name,
            path: None,
            text: RopeText::new(),
            cursor: Cursor::origin(),
            history: UndoHistory::new(),
            modified: false,
            yank_register: String::new(),
        }
    }

    /// Creates a buffer from content.
    pub fn from_content(id: BufferId, name: String, content: &str) -> Self {
        Self {
            id,
            name,
            path: None,
            text: RopeText::from_str(content),
            cursor: Cursor::origin(),
            history: UndoHistory::new(),
            modified: false,
            yank_register: String::new(),
        }
    }

    pub fn id(&self) -> BufferId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    pub fn set_path(&mut self, path: String) {
        self.path = Some(path);
    }

    pub fn version(&self) -> BufferVersion {
        self.text.version()
    }

    pub fn cursor(&self) -> Cursor {
        self.cursor
    }

    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }

    /// Sets the cursor position without clamping.
    /// Use this for insert mode where cursor can be at end of line.
    pub fn set_cursor_position(&mut self, pos: LineCol) {
        self.cursor.position = pos;
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn mark_saved(&mut self) {
        self.modified = false;
    }

    pub fn line_count(&self) -> usize {
        self.text.line_count()
    }

    pub fn line(&self, idx: usize) -> Option<String> {
        self.text.line(idx)
    }

    pub fn line_len(&self, idx: usize) -> Option<usize> {
        self.text.line_len(idx)
    }

    pub fn content(&self) -> String {
        self.text.to_string()
    }

    pub fn yank_register(&self) -> &str {
        &self.yank_register
    }

    /// Sets the yank register content.
    pub fn set_yank_register(&mut self, content: String) {
        self.yank_register = content;
    }

    /// Inserts a character at the cursor position.
    pub fn insert_char(&mut self, ch: char) {
        let pos = self.cursor.position;
        if self.text.insert_char(pos, ch) {
            let mut group = UndoGroup::new();
            group.push(EditOperation::Insert {
                pos,
                text: ch.to_string(),
            });
            self.history.push(group);
            self.cursor.position.col += 1;
            self.modified = true;
        }
    }

    /// Replaces the character at the cursor position (for Replace mode).
    pub fn replace_char(&mut self, ch: char) {
        let pos = self.cursor.position;
        let line_idx = pos.line as usize;
        let col_idx = pos.col as usize;
        
        // Get current line to check if we're at end
        let line = match self.text.line(line_idx) {
            Some(l) => l,
            None => return,
        };
        
        let line_len = line.chars().count();
        
        if col_idx >= line_len {
            // At or past end of line, just insert
            self.insert_char(ch);
        } else {
            // Delete current char and insert new one
            let mut group = UndoGroup::new();
            
            // Get the old char for undo
            let old_char: String = line.chars().nth(col_idx).map(|c| c.to_string()).unwrap_or_default();
            
            // Delete the old char
            if !old_char.is_empty() && self.text.delete_char(pos) {
                group.push(EditOperation::Delete {
                    pos,
                    text: old_char,
                });
            }
            
            // Insert new char
            if self.text.insert_char(pos, ch) {
                group.push(EditOperation::Insert {
                    pos,
                    text: ch.to_string(),
                });
            }
            
            self.history.push(group);
            self.cursor.position.col += 1;
            self.modified = true;
        }
    }

    /// Replaces single character at cursor (r command - no cursor advance).
    pub fn replace_single_char(&mut self, ch: char) {
        let pos = self.cursor.position;
        let line_idx = pos.line as usize;
        let col_idx = pos.col as usize;
        
        // Get current line to check bounds
        let line = match self.text.line(line_idx) {
            Some(l) => l,
            None => return,
        };
        
        let line_len = line.chars().count();
        
        if col_idx >= line_len {
            // At or past end of line, do nothing
            return;
        }
        
        // Delete current char and insert new one
        let mut group = UndoGroup::new();
        
        // Get the old char for undo
        let old_char: String = line.chars().nth(col_idx).map(|c| c.to_string()).unwrap_or_default();
        
        // Delete the old char
        if !old_char.is_empty() && self.text.delete_char(pos) {
            group.push(EditOperation::Delete {
                pos,
                text: old_char,
            });
        }
        
        // Insert new char
        if self.text.insert_char(pos, ch) {
            group.push(EditOperation::Insert {
                pos,
                text: ch.to_string(),
            });
        }
        
        self.history.push(group);
        // Do NOT advance cursor for single char replace
        self.modified = true;
    }

    /// Inserts a newline at the cursor position.
    pub fn insert_newline(&mut self) {
        let pos = self.cursor.position;
        if self.text.insert_char(pos, '\n') {
            let mut group = UndoGroup::new();
            group.push(EditOperation::Insert {
                pos,
                text: "\n".to_string(),
            });
            self.history.push(group);
            self.cursor.position.line += 1;
            self.cursor.position.col = 0;
            self.modified = true;
        }
    }

    /// Deletes the character before the cursor (backspace).
    pub fn delete_char_before(&mut self) {
        if self.cursor.position.col > 0 {
            let del_pos = LineCol::new(
                self.cursor.position.line,
                self.cursor.position.col - 1,
            );
            if let Some(ch) = self.char_at(del_pos) {
                if self.text.delete_char(del_pos) {
                    let mut group = UndoGroup::new();
                    group.push(EditOperation::Delete {
                        pos: del_pos,
                        text: ch.to_string(),
                    });
                    self.history.push(group);
                    self.cursor.position.col -= 1;
                    self.modified = true;
                }
            }
        } else if self.cursor.position.line > 0 {
            let prev_line = self.cursor.position.line - 1;
            let prev_len = self.line_len(prev_line as usize).unwrap_or(0);
            let del_pos = LineCol::new(prev_line, prev_len as u32);
            if self.text.delete_char(del_pos) {
                let mut group = UndoGroup::new();
                group.push(EditOperation::Delete {
                    pos: del_pos,
                    text: "\n".to_string(),
                });
                self.history.push(group);
                self.cursor.position.line = prev_line;
                self.cursor.position.col = prev_len as u32;
                self.modified = true;
            }
        }
    }

    /// Deletes the character at the cursor (x command).
    pub fn delete_char_at(&mut self) {
        let pos = self.cursor.position;
        if let Some(ch) = self.char_at(pos) {
            if self.text.delete_char(pos) {
                let mut group = UndoGroup::new();
                group.push(EditOperation::Delete {
                    pos,
                    text: ch.to_string(),
                });
                self.history.push(group);
                self.clamp_cursor();
                self.modified = true;
            }
        }
    }

    /// Deletes the current line (dd command).
    pub fn delete_line(&mut self) {
        let line_idx = self.cursor.position.line as usize;
        if let Some(content) = self.text.delete_line(line_idx) {
            let mut group = UndoGroup::new();
            group.push(EditOperation::Delete {
                pos: LineCol::new(line_idx as u32, 0),
                text: content.clone(),
            });
            self.history.push(group);
            self.yank_register = content;
            self.clamp_cursor();
            self.modified = true;
        }
    }

    /// Replaces the content of a specific line.
    pub fn replace_line(&mut self, line_idx: usize, new_content: &str) {
        if let Some(old_content) = self.line(line_idx) {
            let start_pos = LineCol::new(line_idx as u32, 0);
            let old_len = old_content.len();
            
            if let Some(start_char) = self.text.line_col_to_char(start_pos) {
                // Remove old content (not including newline)
                self.text.delete_range(start_char, start_char + old_len);
                // Insert new content
                self.text.insert(start_pos, new_content);
                
                let mut group = UndoGroup::new();
                group.push(EditOperation::Delete {
                    pos: start_pos,
                    text: old_content,
                });
                group.push(EditOperation::Insert {
                    pos: start_pos,
                    text: new_content.to_string(),
                });
                self.history.push(group);
                self.modified = true;
            }
        }
    }

    /// Yanks the current line (yy command).
    pub fn yank_line(&mut self) {
        let line_idx = self.cursor.position.line as usize;
        if let Some(content) = self.line(line_idx) {
            self.yank_register = content + "\n";
        }
    }

    /// Pastes after the cursor (p command).
    pub fn paste_after(&mut self) {
        if self.yank_register.is_empty() {
            return;
        }
        let is_line = self.yank_register.ends_with('\n');
        if is_line {
            let next_line = self.cursor.position.line + 1;
            let pos = LineCol::new(next_line, 0);
            if self.text.insert(pos, &self.yank_register) {
                let mut group = UndoGroup::new();
                group.push(EditOperation::Insert {
                    pos,
                    text: self.yank_register.clone(),
                });
                self.history.push(group);
                self.cursor.position.line = next_line;
                self.cursor.position.col = 0;
                self.modified = true;
            }
        } else {
            let pos = LineCol::new(
                self.cursor.position.line,
                self.cursor.position.col + 1,
            );
            if self.text.insert(pos, &self.yank_register) {
                let mut group = UndoGroup::new();
                group.push(EditOperation::Insert {
                    pos,
                    text: self.yank_register.clone(),
                });
                self.history.push(group);
                self.modified = true;
            }
        }
    }

    /// Pastes before the cursor (P command).
    pub fn paste_before(&mut self) {
        if self.yank_register.is_empty() {
            return;
        }
        let is_line = self.yank_register.ends_with('\n');
        if is_line {
            // Insert at start of current line
            let pos = LineCol::new(self.cursor.position.line, 0);
            if self.text.insert(pos, &self.yank_register) {
                let mut group = UndoGroup::new();
                group.push(EditOperation::Insert {
                    pos,
                    text: self.yank_register.clone(),
                });
                self.history.push(group);
                // Cursor stays on the new line
                self.cursor.position.col = 0;
                self.modified = true;
            }
        } else {
            // Insert at cursor position
            let pos = self.cursor.position;
            if self.text.insert(pos, &self.yank_register) {
                let mut group = UndoGroup::new();
                group.push(EditOperation::Insert {
                    pos,
                    text: self.yank_register.clone(),
                });
                self.history.push(group);
                self.modified = true;
            }
        }
    }

    /// Deletes text in a range and yanks it.
    pub fn delete_range(&mut self, start: LineCol, end: LineCol) {
        if start == end {
            return;
        }
        if let Some(text) = self.text.text_range_pos(start, end) {
            if self.text.delete_range_pos(start, end) {
                let mut group = UndoGroup::new();
                group.push(EditOperation::Delete {
                    pos: start,
                    text: text.clone(),
                });
                self.history.push(group);
                self.yank_register = text;
                self.cursor.position = start;
                self.clamp_cursor();
                self.modified = true;
            }
        }
    }

    /// Yanks text in a range without deleting.
    pub fn yank_range(&mut self, start: LineCol, end: LineCol) {
        if let Some(text) = self.text.text_range_pos(start, end) {
            self.yank_register = text;
        }
    }

    /// Indents the current line.
    pub fn indent_line(&mut self) {
        let line_idx = self.cursor.position.line as usize;
        let pos = LineCol::new(line_idx as u32, 0);
        let indent = "    "; // 4 spaces
        if self.text.insert(pos, indent) {
            let mut group = UndoGroup::new();
            group.push(EditOperation::Insert {
                pos,
                text: indent.to_string(),
            });
            self.history.push(group);
            self.modified = true;
        }
    }

    /// Outdents the current line.
    pub fn outdent_line(&mut self) {
        let line_idx = self.cursor.position.line as usize;
        if let Some(line) = self.line(line_idx) {
            let remove_count = line.chars().take(4).take_while(|c| *c == ' ').count();
            if remove_count > 0 {
                let start = LineCol::new(line_idx as u32, 0);
                let end = LineCol::new(line_idx as u32, remove_count as u32);
                let text: String = line.chars().take(remove_count).collect();
                if self.text.delete_range_pos(start, end) {
                    let mut group = UndoGroup::new();
                    group.push(EditOperation::Delete {
                        pos: start,
                        text,
                    });
                    self.history.push(group);
                    self.clamp_cursor();
                    self.modified = true;
                }
            }
        }
    }

    /// Undoes the last change.
    pub fn undo(&mut self) -> bool {
        if let Some(group) = self.history.undo() {
            for op in group.operations() {
                self.apply_operation(op);
            }
            self.modified = true;
            true
        } else {
            false
        }
    }

    /// Redoes the last undone change.
    pub fn redo(&mut self) -> bool {
        if let Some(group) = self.history.redo() {
            for op in group.operations() {
                self.apply_operation(op);
            }
            self.modified = true;
            true
        } else {
            false
        }
    }

    fn apply_operation(&mut self, op: &EditOperation) {
        match op {
            EditOperation::Insert { pos, text } => {
                self.text.insert(*pos, text);
            }
            EditOperation::Delete { pos, text } => {
                if let Some(start_idx) = self.text.line_col_to_char(*pos) {
                    let end = start_idx + text.chars().count();
                    self.text.delete_range(start_idx, end);
                }
            }
        }
    }

    fn char_at(&self, pos: LineCol) -> Option<char> {
        self.line(pos.line as usize)
            .and_then(|line| line.chars().nth(pos.col as usize))
    }

    /// Clamps the cursor to valid positions.
    pub fn clamp_cursor(&mut self) {
        let line_count = self.line_count();
        if line_count == 0 {
            self.cursor.position = LineCol::origin();
            return;
        }
        if self.cursor.position.line as usize >= line_count {
            self.cursor.position.line = (line_count - 1) as u32;
        }
        let line_len = self.line_len(self.cursor.position.line as usize)
            .unwrap_or(0);
        let max_col = if line_len > 0 { line_len - 1 } else { 0 };
        if self.cursor.position.col as usize > max_col {
            self.cursor.position.col = max_col as u32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_undo() {
        let mut buf = Buffer::new(BufferId::new(1), "test".to_string());
        buf.insert_char('a');
        assert_eq!(buf.content(), "a");
        buf.undo();
        assert_eq!(buf.content(), "");
    }

    #[test]
    fn delete_line() {
        let mut buf = Buffer::from_content(
            BufferId::new(1),
            "test".to_string(),
            "line1\nline2",
        );
        buf.delete_line();
        assert_eq!(buf.line_count(), 1);
    }
}
