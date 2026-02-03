#![forbid(unsafe_code)]

mod editor_state;
mod state_error;

pub use editor_state::{CoreAction, EditorState};
pub use state_error::StateError;

