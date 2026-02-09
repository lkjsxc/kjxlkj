/// Mode state machine: transitions, key dispatch,
/// and operator-pending resolution.
mod dispatch;
pub mod transition;

pub use dispatch::{KeyDispatchResult, NormalDispatch};
pub use transition::ModeTransition;
