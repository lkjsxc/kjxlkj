//! Key dispatch to mode handlers.

use crate::insert::dispatch_insert;
use crate::normal::dispatch_normal;
use crate::other_modes::{
    dispatch_command, dispatch_operator_pending, dispatch_replace, dispatch_terminal,
    dispatch_visual,
};
use crate::{HandleResult, ModeState};
use kjxlkj_core_types::{KeyEvent, Mode};

/// Dispatch a key event based on current mode.
pub fn dispatch_key(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    match &state.mode {
        Mode::Normal => dispatch_normal(state, key),
        Mode::Insert => dispatch_insert(state, key),
        Mode::Visual(_) => dispatch_visual(state, key),
        Mode::Command(_) => dispatch_command(state, key),
        Mode::Replace => dispatch_replace(state, key),
        Mode::OperatorPending(_) => dispatch_operator_pending(state, key),
        Mode::InsertNormal => dispatch_insert_normal(state, key),
        Mode::TerminalInsert => dispatch_terminal(state, key),
    }
}

/// Dispatch in insert-normal mode.
fn dispatch_insert_normal(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    let result = dispatch_normal(state, key);
    if matches!(result, HandleResult::Consumed(_)) {
        state.mode = Mode::Insert;
    }
    result
}
