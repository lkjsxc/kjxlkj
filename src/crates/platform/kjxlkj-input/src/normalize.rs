//! Printable key normalization.
//!
//! Stage 2 of the input pipeline.
//! See /docs/spec/architecture/input-decoding.md
//! "Printable Normalization Rules".
//!
//! Rule: Shift+alpha -> uppercase char, shift absorbed.
//! Physical A and Shift+a are indistinguishable downstream.

use kjxlkj_core_types::{Key, KeyModifiers};

/// Normalize a key event by absorbing shift for printable alpha.
///
/// After this function, `Shift+a` is `Key::Char('A')` with
/// `shift = false`.
pub fn normalize_key(
    key: Key,
    mods: KeyModifiers,
) -> (Key, KeyModifiers) {
    match key {
        Key::Char(c) if c.is_ascii_alphabetic() && mods.shift => {
            let upper = c.to_ascii_uppercase();
            let new_mods = KeyModifiers {
                shift: false,
                ..mods
            };
            (Key::Char(upper), new_mods)
        }
        _ => (key, mods),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_alpha_absorbed() {
        let (key, mods) = normalize_key(
            Key::Char('a'),
            KeyModifiers {
                shift: true,
                ctrl: false,
                alt: false,
            },
        );
        assert_eq!(key, Key::Char('A'));
        assert!(!mods.shift);
    }

    #[test]
    fn non_alpha_shift_preserved() {
        let (key, mods) = normalize_key(
            Key::Char('1'),
            KeyModifiers {
                shift: true,
                ctrl: false,
                alt: false,
            },
        );
        assert_eq!(key, Key::Char('1'));
        assert!(mods.shift);
    }

    #[test]
    fn uppercase_already() {
        // Physical A key arrives as Char('A') with shift=false
        // from crossterm when caps lock is on.
        let (key, mods) = normalize_key(
            Key::Char('A'),
            KeyModifiers::default(),
        );
        assert_eq!(key, Key::Char('A'));
        assert!(!mods.shift);
    }
}
