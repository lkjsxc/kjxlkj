//! Auto-pair bracket/quote insertion and skip-over logic.

use serde::{Deserialize, Serialize};

/// Configuration for automatic bracket/quote pairing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoPairConfig {
    pub enabled: bool,
    pub pairs: Vec<(char, char)>,
}

impl Default for AutoPairConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            pairs: default_pairs(),
        }
    }
}

/// Return the default set of auto-closing pairs.
pub fn default_pairs() -> Vec<(char, char)> {
    vec![('(', ')'), ('[', ']'), ('{', '}')]
}

/// If `ch` is an opening character in the config, return its closing counterpart.
pub fn should_auto_close(ch: char, config: &AutoPairConfig) -> Option<char> {
    if !config.enabled {
        return None;
    }
    config
        .pairs
        .iter()
        .find(|(open, _)| *open == ch)
        .map(|(_, close)| *close)
}

/// Returns true if `ch` matches `next_char` as a closing bracket we should skip over.
pub fn should_skip_over(ch: char, next_char: Option<char>, config: &AutoPairConfig) -> bool {
    if !config.enabled {
        return false;
    }
    if let Some(next) = next_char {
        if ch == next {
            return config.pairs.iter().any(|(_, close)| *close == ch);
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_close_paren() {
        let cfg = AutoPairConfig::default();
        assert_eq!(should_auto_close('(', &cfg), Some(')'));
        assert_eq!(should_auto_close('a', &cfg), None);
    }

    #[test]
    fn skip_over_close() {
        let cfg = AutoPairConfig::default();
        assert!(should_skip_over(')', Some(')'), &cfg));
        assert!(!should_skip_over(')', Some('a'), &cfg));
    }

    #[test]
    fn disabled() {
        let cfg = AutoPairConfig {
            enabled: false,
            pairs: default_pairs(),
        };
        assert_eq!(should_auto_close('(', &cfg), None);
        assert!(!should_skip_over(')', Some(')'), &cfg));
    }
}
