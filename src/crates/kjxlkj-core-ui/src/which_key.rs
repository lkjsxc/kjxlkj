//! Which-key style keybinding hints.
//!
//! Displays available keybindings as you type a key sequence.

use std::collections::HashMap;

/// A key hint entry.
#[derive(Debug, Clone)]
pub struct KeyHint {
    /// The key to press (e.g., "w", "d", "<leader>f").
    pub key: String,
    /// Description of what the key does.
    pub desc: String,
    /// Group this key belongs to.
    pub group: Option<String>,
    /// Whether this is a group prefix (has sub-keys).
    pub is_prefix: bool,
    /// Whether this key is hidden from hints.
    pub hidden: bool,
}

impl KeyHint {
    /// Create a new key hint.
    pub fn new(key: impl Into<String>, desc: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            desc: desc.into(),
            group: None,
            is_prefix: false,
            hidden: false,
        }
    }

    /// Set group.
    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Mark as prefix.
    pub fn as_prefix(mut self) -> Self {
        self.is_prefix = true;
        self
    }

    /// Mark as hidden.
    pub fn with_hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }
}

/// Which-key popup position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WhichKeyPosition {
    /// Bottom of screen.
    #[default]
    Bottom,
    /// Top of screen.
    Top,
    /// Center of screen.
    Center,
}

/// Which-key configuration.
#[derive(Debug, Clone)]
pub struct WhichKeyConfig {
    /// Delay before showing popup (ms).
    pub timeout_ms: u32,
    /// Position of popup.
    pub position: WhichKeyPosition,
    /// Show on operator pending.
    pub show_on_operator: bool,
    /// Show on register select.
    pub show_on_register: bool,
    /// Maximum height of popup.
    pub max_height: usize,
    /// Show icons.
    pub icons: bool,
    /// Group separator character.
    pub separator: String,
}

impl Default for WhichKeyConfig {
    fn default() -> Self {
        Self {
            timeout_ms: 500,
            position: WhichKeyPosition::Bottom,
            show_on_operator: true,
            show_on_register: true,
            max_height: 10,
            icons: true,
            separator: "→".to_string(),
        }
    }
}

/// Which-key state.
#[derive(Debug, Default)]
pub struct WhichKey {
    /// Current key prefix being typed.
    prefix: String,
    /// Registered key hints by prefix.
    hints: HashMap<String, Vec<KeyHint>>,
    /// Configuration.
    config: WhichKeyConfig,
    /// Whether popup is visible.
    visible: bool,
    /// Time when prefix started (for timeout).
    started_at: Option<u64>,
}

impl WhichKey {
    /// Create new which-key state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with configuration.
    pub fn with_config(config: WhichKeyConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    /// Register a key hint.
    pub fn register(&mut self, prefix: impl Into<String>, hint: KeyHint) {
        let prefix = prefix.into();
        self.hints.entry(prefix).or_default().push(hint);
    }

    /// Register multiple hints at once.
    pub fn register_group(
        &mut self,
        prefix: impl Into<String>,
        group_name: impl Into<String>,
        hints: Vec<KeyHint>,
    ) {
        let prefix = prefix.into();
        let group = group_name.into();
        let hints_with_group: Vec<_> = hints
            .into_iter()
            .map(|h| h.with_group(group.clone()))
            .collect();
        self.hints.entry(prefix).or_default().extend(hints_with_group);
    }

    /// Start a key sequence.
    pub fn start(&mut self, key: impl Into<String>) {
        self.prefix = key.into();
        self.visible = false;
        self.started_at = Some(0); // TODO: actual timestamp
    }

    /// Add to current prefix.
    pub fn push(&mut self, key: &str) {
        self.prefix.push_str(key);
    }

    /// Clear the prefix.
    pub fn clear(&mut self) {
        self.prefix.clear();
        self.visible = false;
        self.started_at = None;
    }

    /// Get current prefix.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Check if popup should be visible.
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Show the popup.
    pub fn show(&mut self) {
        if !self.prefix.is_empty() {
            self.visible = true;
        }
    }

    /// Hide the popup.
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Get hints for current prefix.
    pub fn get_hints(&self) -> Vec<&KeyHint> {
        self.hints
            .get(&self.prefix)
            .map(|hints| hints.iter().filter(|h| !h.hidden).collect())
            .unwrap_or_default()
    }

    /// Get all hints for a specific prefix.
    pub fn get_hints_for(&self, prefix: &str) -> Vec<&KeyHint> {
        self.hints
            .get(prefix)
            .map(|hints| hints.iter().filter(|h| !h.hidden).collect())
            .unwrap_or_default()
    }

    /// Get configuration.
    pub fn config(&self) -> &WhichKeyConfig {
        &self.config
    }

    /// Get mutable configuration.
    pub fn config_mut(&mut self) -> &mut WhichKeyConfig {
        &mut self.config
    }
}

// ============================================================================
// Command Palette
// ============================================================================

/// A command in the command palette.
#[derive(Debug, Clone)]
pub struct PaletteCommand {
    /// Unique command ID.
    pub id: String,
    /// Display title.
    pub title: String,
    /// Category (e.g., "File", "Edit", "View").
    pub category: Option<String>,
    /// Keybinding hint (e.g., "Ctrl+S").
    pub keybinding: Option<String>,
    /// Whether command is currently enabled.
    pub enabled: bool,
    /// Search score (computed during filtering).
    pub score: i32,
}

impl PaletteCommand {
    /// Create a new palette command.
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            category: None,
            keybinding: None,
            enabled: true,
            score: 0,
        }
    }

    /// Set category.
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Set keybinding hint.
    pub fn with_keybinding(mut self, keybinding: impl Into<String>) -> Self {
        self.keybinding = Some(keybinding.into());
        self
    }

    /// Get full display name.
    pub fn display_name(&self) -> String {
        match &self.category {
            Some(cat) => format!("{}: {}", cat, self.title),
            None => self.title.clone(),
        }
    }
}

/// Command palette state.
#[derive(Debug, Default)]
pub struct CommandPalette {
    /// All registered commands.
    commands: Vec<PaletteCommand>,
    /// Recent commands (IDs).
    recent: Vec<String>,
    /// Current filter query.
    query: String,
    /// Filtered and sorted results.
    filtered: Vec<usize>,
    /// Currently selected index.
    selected: usize,
    /// Whether palette is visible.
    visible: bool,
    /// Maximum recent commands to remember.
    max_recent: usize,
}

impl CommandPalette {
    /// Create new command palette.
    pub fn new() -> Self {
        Self {
            max_recent: 10,
            ..Default::default()
        }
    }

    /// Register a command.
    pub fn register(&mut self, cmd: PaletteCommand) {
        self.commands.push(cmd);
    }

    /// Register multiple commands.
    pub fn register_all(&mut self, cmds: Vec<PaletteCommand>) {
        self.commands.extend(cmds);
    }

    /// Open the palette.
    pub fn open(&mut self) {
        self.visible = true;
        self.query.clear();
        self.selected = 0;
        self.update_filter();
    }

    /// Close the palette.
    pub fn close(&mut self) {
        self.visible = false;
        self.query.clear();
    }

    /// Toggle visibility.
    pub fn toggle(&mut self) {
        if self.visible {
            self.close();
        } else {
            self.open();
        }
    }

    /// Check if visible.
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Set query.
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.selected = 0;
        self.update_filter();
    }

    /// Get current query.
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Type a character.
    pub fn type_char(&mut self, ch: char) {
        self.query.push(ch);
        self.selected = 0;
        self.update_filter();
    }

    /// Backspace.
    pub fn backspace(&mut self) {
        self.query.pop();
        self.selected = 0;
        self.update_filter();
    }

    /// Update filter results.
    fn update_filter(&mut self) {
        self.filtered.clear();

        if self.query.is_empty() {
            // Show recent first, then all
            for recent_id in &self.recent {
                if let Some(idx) = self.commands.iter().position(|c| &c.id == recent_id) {
                    self.filtered.push(idx);
                }
            }
            for (idx, cmd) in self.commands.iter().enumerate() {
                if cmd.enabled && !self.filtered.contains(&idx) {
                    self.filtered.push(idx);
                }
            }
        } else {
            // Fuzzy filter
            let query_lower = self.query.to_lowercase();
            let mut scored: Vec<_> = self
                .commands
                .iter()
                .enumerate()
                .filter(|(_, cmd)| cmd.enabled)
                .filter_map(|(idx, cmd)| {
                    let score = fuzzy_score(&cmd.display_name().to_lowercase(), &query_lower)?;
                    Some((idx, score))
                })
                .collect();

            scored.sort_by(|a, b| b.1.cmp(&a.1));
            self.filtered = scored.into_iter().map(|(idx, _)| idx).collect();
        }
    }

    /// Get filtered commands.
    pub fn results(&self) -> Vec<&PaletteCommand> {
        self.filtered
            .iter()
            .filter_map(|&idx| self.commands.get(idx))
            .collect()
    }

    /// Get selected index.
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// Get selected command.
    pub fn selected_command(&self) -> Option<&PaletteCommand> {
        self.filtered
            .get(self.selected)
            .and_then(|&idx| self.commands.get(idx))
    }

    /// Move selection up.
    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    /// Move selection down.
    pub fn move_down(&mut self) {
        if self.selected + 1 < self.filtered.len() {
            self.selected += 1;
        }
    }

    /// Execute selected command (returns command ID).
    pub fn execute(&mut self) -> Option<String> {
        let cmd = self.selected_command()?;
        let id = cmd.id.clone();

        // Add to recent
        self.recent.retain(|r| r != &id);
        self.recent.insert(0, id.clone());
        if self.recent.len() > self.max_recent {
            self.recent.pop();
        }

        self.close();
        Some(id)
    }
}

/// Simple fuzzy scoring (returns None if no match).
fn fuzzy_score(text: &str, query: &str) -> Option<i32> {
    let mut score = 0i32;
    let mut query_chars = query.chars().peekable();
    let mut last_match_idx = 0;

    for (idx, ch) in text.chars().enumerate() {
        if let Some(&next_query) = query_chars.peek() {
            if ch == next_query {
                query_chars.next();
                // Bonus for consecutive matches
                if idx == last_match_idx + 1 {
                    score += 10;
                }
                // Bonus for start of word
                if idx == 0 || text.chars().nth(idx - 1).map(|c| c == ' ' || c == ':').unwrap_or(false) {
                    score += 5;
                }
                score += 1;
                last_match_idx = idx;
            }
        }
    }

    if query_chars.peek().is_none() {
        Some(score)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_hint_new() {
        let hint = KeyHint::new("w", "Save file");
        assert_eq!(hint.key, "w");
        assert_eq!(hint.desc, "Save file");
        assert!(!hint.is_prefix);
    }

    #[test]
    fn test_which_key_register() {
        let mut wk = WhichKey::new();
        wk.register("", KeyHint::new("g", "Go to...").as_prefix());
        wk.register("g", KeyHint::new("g", "Go to top"));
        wk.register("g", KeyHint::new("d", "Go to definition"));

        let hints = wk.get_hints_for("g");
        assert_eq!(hints.len(), 2);
    }

    #[test]
    fn test_which_key_prefix() {
        let mut wk = WhichKey::new();
        wk.register("", KeyHint::new("g", "Go to..."));
        wk.register("g", KeyHint::new("g", "Top of file"));

        wk.start("g");
        assert_eq!(wk.prefix(), "g");

        let hints = wk.get_hints();
        assert_eq!(hints.len(), 1);
    }

    #[test]
    fn test_which_key_visibility() {
        let mut wk = WhichKey::new();
        assert!(!wk.is_visible());

        wk.start("g");
        wk.show();
        assert!(wk.is_visible());

        wk.hide();
        assert!(!wk.is_visible());

        wk.clear();
        assert_eq!(wk.prefix(), "");
    }

    #[test]
    fn test_palette_command() {
        let cmd = PaletteCommand::new("file.save", "Save File")
            .with_category("File")
            .with_keybinding("Ctrl+S");

        assert_eq!(cmd.display_name(), "File: Save File");
        assert_eq!(cmd.keybinding, Some("Ctrl+S".to_string()));
    }

    #[test]
    fn test_palette_open_close() {
        let mut palette = CommandPalette::new();
        assert!(!palette.is_visible());

        palette.open();
        assert!(palette.is_visible());

        palette.close();
        assert!(!palette.is_visible());
    }

    #[test]
    fn test_palette_filter() {
        let mut palette = CommandPalette::new();
        palette.register(PaletteCommand::new("file.save", "Save File").with_category("File"));
        palette.register(PaletteCommand::new("file.open", "Open File").with_category("File"));
        palette.register(PaletteCommand::new("edit.undo", "Undo").with_category("Edit"));

        palette.open();
        assert_eq!(palette.results().len(), 3);

        palette.set_query("save");
        let results = palette.results();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "file.save");
    }

    #[test]
    fn test_palette_navigation() {
        let mut palette = CommandPalette::new();
        palette.register(PaletteCommand::new("cmd1", "Command 1"));
        palette.register(PaletteCommand::new("cmd2", "Command 2"));
        palette.register(PaletteCommand::new("cmd3", "Command 3"));

        palette.open();
        assert_eq!(palette.selected(), 0);

        palette.move_down();
        assert_eq!(palette.selected(), 1);

        palette.move_down();
        assert_eq!(palette.selected(), 2);

        palette.move_down();
        assert_eq!(palette.selected(), 2); // Can't go past end

        palette.move_up();
        assert_eq!(palette.selected(), 1);
    }

    #[test]
    fn test_palette_execute() {
        let mut palette = CommandPalette::new();
        palette.register(PaletteCommand::new("file.save", "Save File"));

        palette.open();
        let id = palette.execute();
        assert_eq!(id, Some("file.save".to_string()));
        assert!(!palette.is_visible());
        assert_eq!(palette.recent.len(), 1);
    }

    #[test]
    fn test_fuzzy_score() {
        assert!(fuzzy_score("save file", "sav").is_some());
        assert!(fuzzy_score("save file", "sf").is_some());
        assert!(fuzzy_score("save file", "xyz").is_none());
    }
}
