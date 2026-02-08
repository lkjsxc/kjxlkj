//! Shared types used across all kjxlkj crates.
//!
//! This crate defines identifiers, enums, color types, cell attributes,
//! key representations, action types, and mode definitions that form the
//! shared vocabulary of the editor.

mod action;
mod cell;
mod color;
mod ids;
mod key;
mod mode;
mod motion;
mod operator;
mod register;
mod service;
mod text_object;

pub use action::{Action, CommandKind as ActionCommandKind, InsertPosition};
pub use cell::{Cell, CellAttrs};
pub use color::Color;
pub use ids::{BufferId, TabId, TerminalId, WindowId};
pub use key::{Key, KeyCode, KeyModifiers};
pub use mode::{CommandKind, Mode, VisualKind};
pub use motion::{Direction, Motion, ScrollDirection};
pub use operator::Operator;
pub use register::{Register, RegisterName};
pub use service::{ServiceRequest, ServiceResponse};
pub use text_object::{TextObject, TextObjectKind, TextObjectScope};
