use crate::editor::EditorState;
/// Expression register and Ctrl-R insert-mode register insertion.
use kjxlkj_core_edit::RegisterName;

impl EditorState {
    /// Handle Ctrl-R in insert mode: insert register content.
    /// For '=' register, evaluates `last_inserted_text` as an expression.
    pub(crate) fn insert_register_prompt(&mut self) {
        // We use a two-phase approach: set pending_insert_register flag.
        // Next char typed selects the register.
        self.insert_register_pending = true;
    }

    /// Handle the register selector char after Ctrl-R in insert mode.
    pub(crate) fn handle_insert_register(&mut self, c: char) {
        self.insert_register_pending = false;
        if c == '=' {
            // Expression register: evaluate last_ex_command as expression.
            let expr = self.last_ex_command.clone();
            if !expr.is_empty() {
                match crate::expr_eval::eval_expression(&expr) {
                    Ok(result) => self.insert_text(&result),
                    Err(e) => self.notify_error(&format!("E15: {e}")),
                }
            }
            return;
        }
        // Standard register insertion.
        let reg_name = match c {
            'a'..='z' | 'A'..='Z' => RegisterName::Named(c.to_ascii_lowercase()),
            '0'..='9' => RegisterName::Numbered(c as u8 - b'0'),
            '"' => RegisterName::Unnamed,
            '+' => RegisterName::Clipboard,
            '*' => RegisterName::Selection,
            '/' => RegisterName::LastSearch,
            '%' | '#' | ':' | '.' => {
                if let Some(reg) = self.read_special_register(c) {
                    self.insert_text(&reg.content);
                }
                return;
            }
            _ => return,
        };
        if let Some(reg) = self.registers.get(reg_name) {
            let text = reg.content.clone();
            self.insert_text(&text);
        }
    }
}
