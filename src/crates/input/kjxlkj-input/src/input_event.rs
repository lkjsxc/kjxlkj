use kjxlkj_core_types::Key;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputEvent {
    Key(Key),
    Resize { cols: u16, rows: u16 },
}

