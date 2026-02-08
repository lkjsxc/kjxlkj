//! User-defined commands infrastructure.

/// A user-defined ex command.
#[derive(Debug, Clone)]
pub struct UserCommandDef {
    /// Command name (must start with uppercase).
    pub name: String,
    /// Replacement text (the command to execute).
    pub replacement: String,
    /// Whether command accepts a range.
    pub range: bool,
    /// Whether command accepts a count.
    pub count: bool,
    /// Whether command accepts a bang.
    pub bang: bool,
    /// Completion type (empty = none).
    pub complete: String,
    /// Brief description.
    pub description: String,
    /// Buffer-local command.
    pub buffer_local: bool,
}

/// Registry of user commands.
#[derive(Debug, Default)]
pub struct UserCommandRegistry {
    commands: Vec<UserCommandDef>,
}

impl UserCommandRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add or replace a user command.
    pub fn add(&mut self, cmd: UserCommandDef) {
        self.commands
            .retain(|c| c.name != cmd.name);
        self.commands.push(cmd);
    }

    /// Remove a user command by name.
    pub fn remove(&mut self, name: &str) {
        self.commands.retain(|c| c.name != name);
    }

    /// Find a user command by name.
    pub fn find(
        &self,
        name: &str,
    ) -> Option<&UserCommandDef> {
        self.commands
            .iter()
            .find(|c| c.name == name)
    }

    /// List all user commands.
    pub fn list(&self) -> &[UserCommandDef] {
        &self.commands
    }
}

/// Parse a `:command` definition.
/// Format: `:command[-opts] Name replacement`
pub fn parse_user_command(
    args: &str,
) -> Option<UserCommandDef> {
    let args = args.trim();
    if args.is_empty() {
        return None;
    }
    // Find the command name (first word starting
    // with uppercase).
    let parts: Vec<&str> = args.splitn(2, |c: char| {
        c.is_whitespace()
    }).collect();
    if parts.is_empty() {
        return None;
    }
    let name = parts[0];
    if !name
        .chars()
        .next()
        .map_or(false, |c| c.is_uppercase())
    {
        return None;
    }
    let replacement = if parts.len() > 1 {
        parts[1].to_string()
    } else {
        String::new()
    };
    Some(UserCommandDef {
        name: name.to_string(),
        replacement,
        range: false,
        count: false,
        bang: false,
        complete: String::new(),
        description: String::new(),
        buffer_local: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_command() {
        let cmd = parse_user_command(
            "Greet echo hello",
        );
        assert!(cmd.is_some());
        let cmd = cmd.unwrap();
        assert_eq!(cmd.name, "Greet");
        assert_eq!(cmd.replacement, "echo hello");
    }

    #[test]
    fn registry_add_find() {
        let mut reg = UserCommandRegistry::new();
        reg.add(UserCommandDef {
            name: "Hello".to_string(),
            replacement: "echo hi".to_string(),
            range: false,
            count: false,
            bang: false,
            complete: String::new(),
            description: String::new(),
            buffer_local: false,
        });
        assert!(reg.find("Hello").is_some());
        assert!(reg.find("World").is_none());
    }
}
