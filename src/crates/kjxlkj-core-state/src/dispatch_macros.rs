//! Macro recording and playback.

use crate::dispatch_intent;
use crate::EditorState;

/// Handle macro toggle record: start/stop recording.
pub(crate) fn dispatch_macro_toggle(
    state: &mut EditorState,
    reg: char,
) {
    if let Some((rec_reg, intents)) =
        state.macro_recording.take()
    {
        state.macros.insert(rec_reg, intents);
        state.message =
            Some(format!("Recorded @{}", rec_reg));
    } else {
        state.macro_recording = Some((reg, Vec::new()));
        state.message =
            Some(format!("Recording @{}...", reg));
    }
}

/// Play a macro from register.
pub(crate) fn dispatch_macro_play(
    state: &mut EditorState,
    reg: char,
) {
    if let Some(intents) =
        state.macros.get(&reg).cloned()
    {
        state.last_macro = Some(reg);
        let was_recording = state.macro_recording.take();
        for intent in intents {
            dispatch_intent(state, intent);
        }
        state.macro_recording = was_recording;
    } else {
        state.message =
            Some(format!("Empty macro @{}", reg));
    }
}

/// Repeat last played macro.
pub(crate) fn dispatch_macro_repeat_last(
    state: &mut EditorState,
) {
    if let Some(reg) = state.last_macro {
        if let Some(intents) =
            state.macros.get(&reg).cloned()
        {
            let was_recording =
                state.macro_recording.take();
            for intent in intents {
                dispatch_intent(state, intent);
            }
            state.macro_recording = was_recording;
        }
    } else {
        state.message = Some("No previous macro".into());
    }
}
