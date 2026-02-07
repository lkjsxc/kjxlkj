//! kjxlkj-core-types: foundational types shared across all crates.

pub mod action;
pub mod contracts;
pub mod error;
pub mod event;
pub mod latency;
pub mod motion;
pub mod operator;
pub mod text_object;
pub mod types;

pub use action::EditorAction;
pub use contracts::{ContractChecker, ContractLevel, Violation};
pub use error::EditorError;
pub use event::{EditorEvent, KeyCode, KeyEvent, Modifiers, ServiceMsg};
pub use latency::{LatencyBudget, LatencyProbe, TimingGuard};
pub use motion::Motion;
pub use operator::{Operator, OperatorTarget};
pub use text_object::{TextObjectScope, TextObjectType};
pub use types::{Axis, BufferId, BufferVersion, Direction, Mode, Position, Range, WindowId};
