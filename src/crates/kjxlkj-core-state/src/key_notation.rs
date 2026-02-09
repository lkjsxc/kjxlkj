/// Key notation parsing (`<C-a>`, `<CR>`, etc.) into Key values.
use kjxlkj_core_types::{Key, KeyCode, Modifier};

/// Convert a key-notation string into a Vec of Key.
pub(crate) fn parse_key_notation(s: &str) -> Vec<Key> {
    let mut keys = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'<' {
            if let Some(end) = s[i..].find('>') {
                let notation = &s[i + 1..i + end];
                keys.push(parse_single_notation(notation));
                i += end + 1;
                continue;
            }
        }
        let ch = s[i..].chars().next().unwrap();
        keys.push(Key::char(ch));
        i += ch.len_utf8();
    }

    keys
}

/// Parse a single `<...>` notation into a Key.
fn parse_single_notation(notation: &str) -> Key {
    let lower = notation.to_lowercase();
    let simple = |code: KeyCode| Key::new(code, Modifier::NONE);

    match lower.as_str() {
        "cr" | "enter" | "return" => simple(KeyCode::Enter),
        "esc" | "escape" => Key::esc(),
        "bs" | "backspace" => simple(KeyCode::Backspace),
        "tab" => simple(KeyCode::Tab),
        "space" => Key::char(' '),
        "up" => simple(KeyCode::Up),
        "down" => simple(KeyCode::Down),
        "left" => simple(KeyCode::Left),
        "right" => simple(KeyCode::Right),
        "home" => simple(KeyCode::Home),
        "end" => simple(KeyCode::End),
        "pageup" => simple(KeyCode::PageUp),
        "pagedown" => simple(KeyCode::PageDown),
        "del" | "delete" => simple(KeyCode::Delete),
        "f1" => simple(KeyCode::F(1)),
        "f2" => simple(KeyCode::F(2)),
        "f3" => simple(KeyCode::F(3)),
        "f4" => simple(KeyCode::F(4)),
        "f5" => simple(KeyCode::F(5)),
        "f6" => simple(KeyCode::F(6)),
        "f7" => simple(KeyCode::F(7)),
        "f8" => simple(KeyCode::F(8)),
        "f9" => simple(KeyCode::F(9)),
        "f10" => simple(KeyCode::F(10)),
        "f11" => simple(KeyCode::F(11)),
        "f12" => simple(KeyCode::F(12)),
        _ => {
            if let Some(ch_str) = lower.strip_prefix("c-") {
                if let Some(ch) = ch_str.chars().next() {
                    return Key::ctrl(ch);
                }
            }
            Key::char('<')
        }
    }
}
