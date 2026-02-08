//! Audio/bell configuration: visual bell, system bell,
//! and audio feedback settings.

/// Bell mode for error notifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BellMode {
    /// No bell at all.
    None,
    /// Terminal system bell (BEL character).
    System,
    /// Visual bell (flash screen).
    Visual,
    /// Both system and visual bell.
    Both,
}

impl Default for BellMode {
    fn default() -> Self {
        BellMode::None
    }
}

/// Audio/bell configuration.
#[derive(Debug, Clone)]
pub struct AudioConfig {
    /// Bell mode.
    pub bell: BellMode,
    /// Visual bell duration in milliseconds.
    pub visual_bell_duration_ms: u64,
    /// Whether error sounds are enabled.
    pub error_bells: bool,
    /// Bell on search wrap-around.
    pub search_bell: bool,
    /// Bell on reaching buffer edges.
    pub edge_bell: bool,
}

impl AudioConfig {
    pub fn new() -> Self {
        Self {
            bell: BellMode::None,
            visual_bell_duration_ms: 100,
            error_bells: false,
            search_bell: false,
            edge_bell: false,
        }
    }

    /// Generate the terminal sequence for the configured bell.
    pub fn bell_sequence(&self) -> &str {
        match self.bell {
            BellMode::None => "",
            BellMode::System | BellMode::Both => "\x07",
            BellMode::Visual => "",
        }
    }

    /// Whether any bell is active.
    pub fn has_bell(&self) -> bool {
        self.bell != BellMode::None
    }

    /// Parse bell mode from option string.
    pub fn parse_bell_mode(s: &str) -> BellMode {
        match s {
            "none" | "off" => BellMode::None,
            "system" | "beep" => BellMode::System,
            "visual" | "flash" => BellMode::Visual,
            "both" | "all" => BellMode::Both,
            _ => BellMode::None,
        }
    }

    /// Set from `:set belloff=...` option.
    pub fn set_belloff(&mut self, value: &str) {
        if value == "all" {
            self.error_bells = false;
            self.search_bell = false;
            self.edge_bell = false;
            return;
        }
        for part in value.split(',') {
            match part.trim() {
                "error" => self.error_bells = false,
                "search" => self.search_bell = false,
                "esc" | "edge" => self.edge_bell = false,
                _ => {}
            }
        }
    }
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_no_bell() {
        let cfg = AudioConfig::new();
        assert!(!cfg.has_bell());
        assert_eq!(cfg.bell_sequence(), "");
    }

    #[test]
    fn system_bell() {
        let mut cfg = AudioConfig::new();
        cfg.bell = BellMode::System;
        assert!(cfg.has_bell());
        assert_eq!(cfg.bell_sequence(), "\x07");
    }

    #[test]
    fn parse_bell_modes() {
        assert_eq!(AudioConfig::parse_bell_mode("none"), BellMode::None);
        assert_eq!(AudioConfig::parse_bell_mode("visual"), BellMode::Visual);
        assert_eq!(AudioConfig::parse_bell_mode("system"), BellMode::System);
    }

    #[test]
    fn belloff_all() {
        let mut cfg = AudioConfig::new();
        cfg.error_bells = true;
        cfg.search_bell = true;
        cfg.set_belloff("all");
        assert!(!cfg.error_bells);
        assert!(!cfg.search_bell);
    }
}
