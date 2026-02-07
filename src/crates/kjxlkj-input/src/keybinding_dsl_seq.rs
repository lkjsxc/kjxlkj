//! Key sequence parsing and validation.

use crate::keybinding_dsl::{parse_key_notation, KeyChord};

/// Parse a key sequence notation like `<C-w>j` or `<Leader>ff`.
pub fn parse_key_sequence(notation: &str) -> Vec<KeyChord> {
    let mut result = Vec::new();
    let mut chars = notation.chars().peekable();
    while let Some(&c) = chars.peek() {
        if c == '<' {
            let mut bracket = String::from('<');
            chars.next();
            while let Some(&ch) = chars.peek() {
                bracket.push(ch);
                chars.next();
                if ch == '>' {
                    break;
                }
            }
            if let Some(chord) = parse_key_notation(&bracket) {
                result.push(chord);
            }
        } else {
            chars.next();
            result.push(KeyChord {
                key: c.to_string(),
                ctrl: false,
                alt: false,
                shift: false,
            });
        }
    }
    result
}

/// Validate a key sequence notation string.
pub fn validate_key_sequence(notation: &str) -> Result<(), String> {
    if notation.is_empty() {
        return Err("empty key sequence".into());
    }
    let chords = parse_key_sequence(notation);
    if chords.is_empty() {
        return Err(format!("could not parse: {notation}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keybinding_dsl::{parse_key_notation, resolve_special};

    #[test]
    fn parse_plain_key() {
        let k = parse_key_notation("j").unwrap();
        assert_eq!(k.key, "j");
        assert!(!k.ctrl && !k.alt && !k.shift);
    }

    #[test]
    fn parse_ctrl_key() {
        let k = parse_key_notation("<C-s>").unwrap();
        assert_eq!(k.key, "s");
        assert!(k.ctrl);
    }

    #[test]
    fn parse_special() {
        let k = parse_key_notation("<Space>").unwrap();
        assert_eq!(k.key, "Space");
    }

    #[test]
    fn parse_sequence() {
        let seq = parse_key_sequence("<C-w>j");
        assert_eq!(seq.len(), 2);
        assert!(seq[0].ctrl);
        assert_eq!(seq[1].key, "j");
    }

    #[test]
    fn validate_empty() {
        assert!(validate_key_sequence("").is_err());
    }

    #[test]
    fn resolve_f_keys() {
        assert_eq!(resolve_special("F1"), Some("F1".into()));
        assert_eq!(resolve_special("f12"), Some("F12".into()));
        assert_eq!(resolve_special("f13"), None);
    }

    #[test]
    fn parse_multi_modifier() {
        let k = parse_key_notation("<C-A-x>").unwrap();
        assert!(k.ctrl && k.alt);
        assert_eq!(k.key, "x");
    }
}
