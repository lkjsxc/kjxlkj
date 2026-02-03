#![forbid(unsafe_code)]

use kjxlkj_core_edit::Edit;

#[derive(Clone, Debug, Default)]
pub struct UndoStack {
    undo: Vec<Vec<Edit>>,
    redo: Vec<Vec<Edit>>,
}

impl UndoStack {
    pub fn push_transaction(&mut self, edits: Vec<Edit>) {
        if edits.is_empty() {
            return;
        }
        self.undo.push(edits);
        self.redo.clear();
    }

    pub fn pop_undo(&mut self) -> Option<Vec<Edit>> {
        let tx = self.undo.pop()?;
        self.redo.push(tx.clone());
        Some(tx)
    }

    pub fn pop_redo(&mut self) -> Option<Vec<Edit>> {
        let tx = self.redo.pop()?;
        self.undo.push(tx.clone());
        Some(tx)
    }
}

