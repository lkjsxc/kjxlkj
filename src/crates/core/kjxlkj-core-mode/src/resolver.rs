//! Mode transition resolver.
//!
//! Maps (current mode, key) -> new mode deterministically.

use kjxlkj_core_types::Mode;

/// Resolve any explicit mode set from dispatch.
///
/// The transition table is fully specified in
/// /docs/spec/modes/transitions.md.
pub fn resolve_mode_transition(
    current: Mode,
    new_mode: Option<Mode>,
) -> Mode {
    match new_mode {
        Some(m) => m,
        None => current,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_transition_keeps_mode() {
        let m = resolve_mode_transition(Mode::Normal, None);
        assert_eq!(m, Mode::Normal);
    }

    #[test]
    fn explicit_transition_applies() {
        let m =
            resolve_mode_transition(Mode::Normal, Some(Mode::Insert));
        assert_eq!(m, Mode::Insert);
    }
}
