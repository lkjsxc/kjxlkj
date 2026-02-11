//! Normal mode partial key resolution.
//!
//! Handles f/F/t/T + char, r + char, m + char, etc.
//! See /docs/spec/modes/normal.md.

use kjxlkj_core_types::{Action, Key, Mode, Motion};

use crate::normal_g;
use crate::normal_wincmd;
use crate::normal_z;
use crate::pending::{PartialKey, PendingState};

/// Resolve a second key when a partial key is pending.
pub(crate) fn resolve_partial(
    key: &Key,
    pending: &mut PendingState,
) -> (Action, Option<Mode>) {
    let partial = pending.partial.clone();
    match partial {
        PartialKey::G => normal_g::handle_g_key(key, pending),
        PartialKey::Z => normal_z::handle_z_key(key, pending),
        PartialKey::WinCmd => normal_wincmd::handle_wincmd_key(key, pending),
        PartialKey::FindForward => {
            if let Key::Char(c) = key {
                pending.clear();
                (Action::Motion(Motion::FindForward(*c)), None)
            } else {
                pending.clear();
                (Action::Noop, None)
            }
        }
        PartialKey::FindBackward => {
            if let Key::Char(c) = key {
                pending.clear();
                (Action::Motion(Motion::FindBackward(*c)), None)
            } else {
                pending.clear();
                (Action::Noop, None)
            }
        }
        PartialKey::TillForward => {
            if let Key::Char(c) = key {
                pending.clear();
                (Action::Motion(Motion::TillForward(*c)), None)
            } else {
                pending.clear();
                (Action::Noop, None)
            }
        }
        PartialKey::TillBackward => {
            if let Key::Char(c) = key {
                pending.clear();
                (Action::Motion(Motion::TillBackward(*c)), None)
            } else {
                pending.clear();
                (Action::Noop, None)
            }
        }
        PartialKey::ReplaceChar => {
            if let Key::Char(c) = key {
                pending.clear();
                (Action::ReplaceChar(*c), None)
            } else {
                pending.clear();
                (Action::Noop, None)
            }
        }
        PartialKey::Register => {
            if let Key::Char(c) = key {
                // Store selected register, return to
                // normal key dispatch (not partial).
                pending.partial = PartialKey::None;
                pending.register = Some(*c);
                (Action::Noop, None)
            } else {
                pending.clear();
                (Action::Noop, None)
            }
        }
        PartialKey::SetMark => {
            if let Key::Char(c) = key {
                pending.clear();
                (Action::SetMark(*c), None)
            } else { pending.clear(); (Action::Noop, None) }
        }
        PartialKey::GotoMarkLine => {
            if let Key::Char(c) = key {
                pending.clear();
                (Action::GotoMarkLine(*c), None)
            } else { pending.clear(); (Action::Noop, None) }
        }
        PartialKey::GotoMarkExact => {
            if let Key::Char(c) = key {
                pending.clear();
                (Action::GotoMarkExact(*c), None)
            } else { pending.clear(); (Action::Noop, None) }
        }
        PartialKey::MacroRecord => {
            if let Key::Char(c) = key {
                pending.clear();
                (Action::MacroRecordStart(*c), None)
            } else { pending.clear(); (Action::Noop, None) }
        }
        PartialKey::MacroPlay => {
            if let Key::Char(c) = key {
                pending.clear();
                (Action::MacroPlay(*c), None)
            } else { pending.clear(); (Action::Noop, None) }
        }
        _ => {
            pending.clear();
            (Action::Noop, None)
        }
    }
}
