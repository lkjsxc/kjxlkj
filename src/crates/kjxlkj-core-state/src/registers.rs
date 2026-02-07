//! Register management: yank, delete, and named registers.

use std::collections::HashMap;

/// A register entry with its content and line-wise flag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterEntry {
    pub content: String,
    pub linewise: bool,
}

/// Default (unnamed) register.
pub const DEFAULT_REGISTER: char = '"';
/// Yank register.
pub const YANK_REGISTER: char = '0';
/// Small delete register (less than one line).
pub const SMALL_DELETE_REGISTER: char = '-';

/// Yank text into a register.
pub fn yank_to_register(
    registers: &mut HashMap<char, RegisterEntry>,
    reg: char,
    text: &str,
    linewise: bool,
) {
    let entry = RegisterEntry { content: text.to_string(), linewise };
    registers.insert(DEFAULT_REGISTER, entry.clone());
    if reg == DEFAULT_REGISTER {
        registers.insert(YANK_REGISTER, entry);
    } else {
        registers.insert(reg, entry);
    }
}

/// Delete text into a register, rotating numbered registers 1-9.
pub fn delete_to_register(
    registers: &mut HashMap<char, RegisterEntry>,
    reg: char,
    text: &str,
    linewise: bool,
) {
    let entry = RegisterEntry { content: text.to_string(), linewise };
    registers.insert(DEFAULT_REGISTER, entry.clone());
    if reg == DEFAULT_REGISTER {
        if !linewise && !text.contains('\n') {
            registers.insert(SMALL_DELETE_REGISTER, entry);
        } else {
            for i in (2..=9).rev() {
                let prev = char::from_digit(i - 1, 10).unwrap();
                let curr = char::from_digit(i, 10).unwrap();
                if let Some(e) = registers.get(&prev).cloned() {
                    registers.insert(curr, e);
                }
            }
            registers.insert('1', entry);
        }
    } else {
        registers.insert(reg, entry);
    }
}

/// Get a register entry.
pub fn get_register(
    registers: &HashMap<char, RegisterEntry>,
    reg: char,
) -> Option<&RegisterEntry> {
    registers.get(&reg)
}

/// Check if a character is a valid register name.
pub fn is_valid_register(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || "\"+-*/.#:_%".contains(ch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yank_default() {
        let mut regs = HashMap::new();
        yank_to_register(&mut regs, DEFAULT_REGISTER, "hello", false);
        assert_eq!(get_register(&regs, YANK_REGISTER).unwrap().content, "hello");
        assert_eq!(get_register(&regs, DEFAULT_REGISTER).unwrap().content, "hello");
    }

    #[test]
    fn yank_named() {
        let mut regs = HashMap::new();
        yank_to_register(&mut regs, 'a', "text", false);
        assert_eq!(get_register(&regs, 'a').unwrap().content, "text");
        assert_eq!(get_register(&regs, DEFAULT_REGISTER).unwrap().content, "text");
    }

    #[test]
    fn delete_rotates_numbered() {
        let mut regs = HashMap::new();
        delete_to_register(&mut regs, DEFAULT_REGISTER, "first\n", true);
        delete_to_register(&mut regs, DEFAULT_REGISTER, "second\n", true);
        assert_eq!(get_register(&regs, '1').unwrap().content, "second\n");
        assert_eq!(get_register(&regs, '2').unwrap().content, "first\n");
    }

    #[test]
    fn small_delete() {
        let mut regs = HashMap::new();
        delete_to_register(&mut regs, DEFAULT_REGISTER, "x", false);
        assert_eq!(get_register(&regs, SMALL_DELETE_REGISTER).unwrap().content, "x");
    }

    #[test]
    fn valid_registers() {
        assert!(is_valid_register('a'));
        assert!(is_valid_register('"'));
        assert!(!is_valid_register('!'));
    }
}
