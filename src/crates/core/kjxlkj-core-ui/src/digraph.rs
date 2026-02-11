//! Digraph lookup table (RFC 1345 subset).
//! See /docs/spec/modes/insert/input/insert-digraphs.md.

/// Look up a digraph by its two-character code.
/// Returns the resulting Unicode character, or None if undefined.
pub fn lookup(c1: char, c2: char) -> Option<char> {
    let key = (c1, c2);
    DIGRAPHS.iter().find(|&&(a, b, _)| (a, b) == key).map(|&(_, _, ch)| ch)
}

/// Core RFC 1345 digraph table (frequently used subset).
static DIGRAPHS: &[(char, char, char)] = &[
    // Latin accented
    ('a', ':', '\u{00E4}'), ('o', ':', '\u{00F6}'), ('u', ':', '\u{00FC}'),
    ('A', ':', '\u{00C4}'), ('O', ':', '\u{00D6}'), ('U', ':', '\u{00DC}'),
    ('a', '\'', '\u{00E1}'), ('e', '\'', '\u{00E9}'), ('i', '\'', '\u{00ED}'),
    ('o', '\'', '\u{00F3}'), ('u', '\'', '\u{00FA}'),
    ('a', '!', '\u{00E0}'), ('e', '!', '\u{00E8}'), ('i', '!', '\u{00EC}'),
    ('o', '!', '\u{00F2}'), ('u', '!', '\u{00F9}'),
    ('a', '>', '\u{00E2}'), ('e', '>', '\u{00EA}'), ('i', '>', '\u{00EE}'),
    ('o', '>', '\u{00F4}'), ('u', '>', '\u{00FB}'),
    ('n', '~', '\u{00F1}'), ('N', '~', '\u{00D1}'),
    ('c', ',', '\u{00E7}'), ('C', ',', '\u{00C7}'),
    ('s', 's', '\u{00DF}'), // German sharp s
    // Symbols
    ('C', 'o', '\u{00A9}'), // ©
    ('R', 'g', '\u{00AE}'), // ®
    ('T', 'M', '\u{2122}'), // ™
    ('1', '4', '\u{00BC}'), // ¼
    ('1', '2', '\u{00BD}'), // ½
    ('3', '4', '\u{00BE}'), // ¾
    ('<', '<', '\u{00AB}'), // «
    ('>', '>', '\u{00BB}'), // »
    ('-', '>', '\u{2192}'), // →
    ('<', '-', '\u{2190}'), // ←
    ('!', '=', '\u{2260}'), // ≠
    ('>', '=', '\u{2265}'), // ≥
    ('<', '=', '\u{2264}'), // ≤
    ('E', 'u', '\u{20AC}'), // €
    ('P', 'd', '\u{00A3}'), // £
    ('Y', 'e', '\u{00A5}'), // ¥
    ('D', 'G', '\u{00B0}'), // °
    ('M', 'y', '\u{00B5}'), // µ
    ('P', 'I', '\u{00B6}'), // ¶
    ('S', 'E', '\u{00A7}'), // §
    ('.', 'M', '\u{00B7}'), // ·
    ('N', 'O', '\u{2116}'), // №
    ('+', '-', '\u{00B1}'), // ±
    ('*', '*', '\u{2022}'), // •
    ('.', '.', '\u{2026}'), // …
    // Math / Greek
    ('*', 'a', '\u{03B1}'), // α
    ('*', 'b', '\u{03B2}'), // β
    ('*', 'g', '\u{03B3}'), // γ
    ('*', 'd', '\u{03B4}'), // δ
    ('*', 'p', '\u{03C0}'), // π
    ('*', 'l', '\u{03BB}'), // λ
    ('*', 's', '\u{03C3}'), // σ
    ('*', 'S', '\u{03A3}'), // Σ
    ('*', 'W', '\u{03A9}'), // Ω
    ('O', 'K', '\u{2713}'), // ✓
    ('X', 'X', '\u{2717}'), // ✗
];

/// Total number of built-in digraphs.
pub fn count() -> usize { DIGRAPHS.len() }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_umlaut() {
        assert_eq!(lookup('a', ':'), Some('\u{00E4}'));
        assert_eq!(lookup('O', ':'), Some('\u{00D6}'));
    }

    #[test]
    fn lookup_copyright() {
        assert_eq!(lookup('C', 'o'), Some('\u{00A9}'));
    }

    #[test]
    fn lookup_arrow() {
        assert_eq!(lookup('-', '>'), Some('\u{2192}'));
        assert_eq!(lookup('<', '-'), Some('\u{2190}'));
    }

    #[test]
    fn lookup_euro() {
        assert_eq!(lookup('E', 'u'), Some('\u{20AC}'));
    }

    #[test]
    fn lookup_unknown() {
        assert_eq!(lookup('z', 'z'), None);
    }

    #[test]
    fn lookup_greek() {
        assert_eq!(lookup('*', 'p'), Some('\u{03C0}'));
        assert_eq!(lookup('*', 'l'), Some('\u{03BB}'));
    }

    #[test]
    fn count_matches() {
        assert!(count() >= 50);
    }

    #[test]
    fn lookup_sharp_s() {
        assert_eq!(lookup('s', 's'), Some('\u{00DF}'));
    }

    #[test]
    fn lookup_checkmark() {
        assert_eq!(lookup('O', 'K'), Some('\u{2713}'));
    }
}
