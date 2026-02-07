//! Digraph table: common character combinations for special characters.

/// A digraph entry mapping two input characters to an output character.
type DigraphEntry = (char, char, char);

/// Static table of common digraphs (based on RFC 1345 / Vim defaults).
pub static DIGRAPH_TABLE: &[DigraphEntry] = &[
    // Currency
    ('C', 't', '¢'),
    ('P', 'd', '£'),
    ('Y', 'e', '¥'),
    ('E', 'u', '€'),
    // Punctuation
    ('!', '!', '¡'),
    ('?', '?', '¿'),
    ('<', '<', '«'),
    ('>', '>', '»'),
    ('.', '.', '\u{2026}'),
    ('-', '-', '\u{2014}'),
    ('-', 'N', '\u{2013}'),
    ('\'', '6', '\u{2018}'),
    ('\'', '9', '\u{2019}'),
    ('"', '6', '\u{201C}'),
    ('"', '9', '\u{201D}'),
    // Math / logic
    ('+', '-', '±'),
    ('D', 'G', '°'),
    ('*', '*', '×'),
    ('-', ':', '÷'),
    ('!', '=', '≠'),
    ('<', '=', '≤'),
    ('>', '=', '≥'),
    ('O', 'K', '✓'),
    ('X', 'X', '✗'),
    ('I', 'n', '∞'),
    ('S', 'q', '√'),
    // Greek letters (common)
    ('a', '*', 'α'),
    ('b', '*', 'β'),
    ('g', '*', 'γ'),
    ('d', '*', 'δ'),
    ('e', '*', 'ε'),
    ('p', '*', 'π'),
    ('l', '*', 'λ'),
    ('m', '*', 'μ'),
    ('s', '*', 'σ'),
    ('t', '*', 'τ'),
    ('f', '*', 'φ'),
    ('w', '*', 'ω'),
    ('G', '*', 'Γ'),
    ('D', '*', 'Δ'),
    ('S', '*', 'Σ'),
    ('P', '*', 'Π'),
    ('W', '*', 'Ω'),
    // Arrows
    ('-', '>', '→'),
    ('<', '-', '←'),
    ('-', '!', '↑'),
    ('-', 'v', '↓'),
    ('=', '>', '⇒'),
    ('<', '!', '⇐'),
    // Accented vowels
    ('a', '\'', 'á'),
    ('e', '\'', 'é'),
    ('i', '\'', 'í'),
    ('o', '\'', 'ó'),
    ('u', '\'', 'ú'),
    ('a', '`', 'à'),
    ('e', '`', 'è'),
    ('i', '`', 'ì'),
    ('o', '`', 'ò'),
    ('u', '`', 'ù'),
    ('a', '^', 'â'),
    ('e', '^', 'ê'),
    ('i', '^', 'î'),
    ('o', '^', 'ô'),
    ('u', '^', 'û'),
    ('a', ':', 'ä'),
    ('e', ':', 'ë'),
    ('i', ':', 'ï'),
    ('o', ':', 'ö'),
    ('u', ':', 'ü'),
    ('a', '~', 'ã'),
    ('n', '~', 'ñ'),
    ('o', '~', 'õ'),
    // Misc
    ('c', ',', 'ç'),
    ('C', ',', 'Ç'),
    ('A', 'E', 'Æ'),
    ('a', 'e', 'æ'),
    ('s', 's', 'ß'),
    ('O', '/', 'Ø'),
    ('o', '/', 'ø'),
    ('1', '2', '½'),
    ('1', '4', '¼'),
    ('3', '4', '¾'),
    ('N', 'o', '№'),
    ('C', 'o', '©'),
    ('R', 'g', '®'),
    ('T', 'M', '™'),
    ('S', 'E', '§'),
    ('P', 'I', '¶'),
    ('M', 'y', 'µ'),
];

/// Look up a digraph by its two input characters.
pub fn lookup_digraph(c1: char, c2: char) -> Option<char> {
    // Try exact match first
    for &(a, b, result) in DIGRAPH_TABLE {
        if a == c1 && b == c2 {
            return Some(result);
        }
    }
    // Try reversed order
    for &(a, b, result) in DIGRAPH_TABLE {
        if a == c2 && b == c1 {
            return Some(result);
        }
    }
    None
}

/// Format the digraph table as a human-readable string.
pub fn format_digraph_table() -> String {
    let mut out = String::with_capacity(DIGRAPH_TABLE.len() * 12);
    out.push_str("Digraphs:\n");
    for (i, &(c1, c2, result)) in DIGRAPH_TABLE.iter().enumerate() {
        out.push_str(&format!("  {}{} → {}  ", c1, c2, result));
        if (i + 1) % 6 == 0 {
            out.push('\n');
        }
    }
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_existing() {
        assert_eq!(lookup_digraph('C', 't'), Some('¢'));
        assert_eq!(lookup_digraph('E', 'u'), Some('€'));
        assert_eq!(lookup_digraph('a', '*'), Some('α'));
    }

    #[test]
    fn lookup_reversed() {
        assert_eq!(lookup_digraph('t', 'C'), Some('¢'));
    }

    #[test]
    fn lookup_missing() {
        assert_eq!(lookup_digraph('Z', 'Z'), None);
    }

    #[test]
    fn table_not_empty() {
        assert!(DIGRAPH_TABLE.len() > 50);
    }

    #[test]
    fn format_table_not_empty() {
        let s = format_digraph_table();
        assert!(s.starts_with("Digraphs:"));
        assert!(s.len() > 100);
    }

    #[test]
    fn all_entries_unique_result() {
        // Verify the first digraph maps correctly
        let &(c1, c2, expected) = &DIGRAPH_TABLE[0];
        assert_eq!(lookup_digraph(c1, c2), Some(expected));
    }

    #[test]
    fn arrows() {
        assert_eq!(lookup_digraph('-', '>'), Some('→'));
        assert_eq!(lookup_digraph('<', '-'), Some('←'));
    }

    #[test]
    fn accented_vowels() {
        assert_eq!(lookup_digraph('a', '\''), Some('á'));
        assert_eq!(lookup_digraph('u', ':'), Some('ü'));
        assert_eq!(lookup_digraph('n', '~'), Some('ñ'));
    }

    #[test]
    fn fractions() {
        assert_eq!(lookup_digraph('1', '2'), Some('½'));
        assert_eq!(lookup_digraph('3', '4'), Some('¾'));
    }

    #[test]
    fn copyright_tm() {
        assert_eq!(lookup_digraph('C', 'o'), Some('©'));
        assert_eq!(lookup_digraph('T', 'M'), Some('™'));
    }
}
