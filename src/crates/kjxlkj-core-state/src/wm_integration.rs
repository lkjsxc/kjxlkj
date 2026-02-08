//! Window manager integration: i3/sway/hyprland
//! awareness and focus coordination.

/// Supported window manager types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WmType {
    /// No WM detected (bare terminal).
    None,
    /// i3 window manager.
    I3,
    /// Sway (Wayland i3 clone).
    Sway,
    /// Hyprland.
    Hyprland,
    /// Generic X11.
    X11,
    /// Generic Wayland.
    Wayland,
}

/// WM integration state.
#[derive(Debug, Clone)]
pub struct WmState {
    /// Detected window manager type.
    pub wm_type: WmType,
    /// Whether to pass unfocusable splits to WM.
    pub wm_navigation: bool,
    /// Whether WM focus events are tracked.
    pub track_focus: bool,
}

impl WmState {
    pub fn new() -> Self {
        Self {
            wm_type: WmType::None,
            wm_navigation: false,
            track_focus: false,
        }
    }

    /// Detect WM from environment.
    pub fn detect() -> Self {
        let wm_type = if std::env::var("SWAYSOCK").is_ok() {
            WmType::Sway
        } else if std::env::var("I3SOCK").is_ok() {
            WmType::I3
        } else if std::env::var("HYPRLAND_INSTANCE_SIGNATURE")
            .is_ok()
        {
            WmType::Hyprland
        } else if std::env::var("WAYLAND_DISPLAY").is_ok() {
            WmType::Wayland
        } else if std::env::var("DISPLAY").is_ok() {
            WmType::X11
        } else {
            WmType::None
        };
        Self {
            wm_type,
            wm_navigation: false,
            track_focus: wm_type != WmType::None,
        }
    }

    /// Build a WM focus command for navigating outside the editor.
    /// Returns None if not applicable.
    pub fn focus_command(
        &self,
        direction: &str,
    ) -> Option<String> {
        match self.wm_type {
            WmType::I3 => Some(format!(
                "i3-msg focus {}",
                direction
            )),
            WmType::Sway => Some(format!(
                "swaymsg focus {}",
                direction
            )),
            WmType::Hyprland => Some(format!(
                "hyprctl dispatch movefocus {}",
                match direction {
                    "left" => "l",
                    "right" => "r",
                    "up" => "u",
                    "down" => "d",
                    other => other,
                }
            )),
            _ => None,
        }
    }

    /// Whether WM navigation is available.
    pub fn has_wm_navigation(&self) -> bool {
        self.wm_navigation && self.wm_type != WmType::None
    }
}

impl Default for WmState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_wm_navigation() {
        let state = WmState::new();
        assert_eq!(state.wm_type, WmType::None);
        assert!(!state.has_wm_navigation());
        assert!(state.focus_command("left").is_none());
    }

    #[test]
    fn i3_focus_command() {
        let mut state = WmState::new();
        state.wm_type = WmType::I3;
        state.wm_navigation = true;
        let cmd = state.focus_command("left").unwrap();
        assert!(cmd.contains("i3-msg focus left"));
    }

    #[test]
    fn hyprland_focus_mapping() {
        let mut state = WmState::new();
        state.wm_type = WmType::Hyprland;
        let cmd = state.focus_command("up").unwrap();
        assert!(cmd.contains("movefocus u"));
    }
}
