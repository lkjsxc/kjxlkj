use kjxlkj_core_types::Mode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputAction {
    Quit,
    SetMode(Mode),
    InsertText(String),
}

