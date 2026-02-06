/// User-defined Ex commands — :command, :delcommand, custom command registry.

use std::collections::HashMap;

/// Attributes for a user-defined command.
#[derive(Debug, Clone, PartialEq)]
pub struct UserCommandDef {
    pub name: String,
    pub replacement: String,
    pub nargs: NArgs,
    pub range_allowed: bool,
    pub bang_allowed: bool,
    pub bar_allowed: bool,
    pub complete: Option<String>,
    pub buffer_local: bool,
}

/// Number of arguments specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NArgs { Zero, One, Any, AtLeastOne, ZeroOrOne }

impl NArgs {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::Zero), "1" => Some(Self::One),
            "*" => Some(Self::Any), "+" => Some(Self::AtLeastOne),
            "?" => Some(Self::ZeroOrOne), _ => None,
        }
    }
    pub fn validate_count(&self, count: usize) -> bool {
        match self {
            Self::Zero => count == 0, Self::One => count == 1,
            Self::Any => true, Self::AtLeastOne => count >= 1,
            Self::ZeroOrOne => count <= 1,
        }
    }
}

/// Registry for user-defined commands.
#[derive(Debug, Default)]
pub struct UserCommandRegistry { commands: HashMap<String, UserCommandDef> }

impl UserCommandRegistry {
    pub fn new() -> Self { Self::default() }

    pub fn define(&mut self, def: UserCommandDef) -> Result<(), String> {
        if def.name.is_empty() || !def.name.chars().next().unwrap().is_uppercase() {
            return Err("User commands must start with uppercase".into());
        }
        self.commands.insert(def.name.clone(), def);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&UserCommandDef> { self.commands.get(name) }

    pub fn remove(&mut self, name: &str) -> bool { self.commands.remove(name).is_some() }

    pub fn list(&self) -> Vec<&UserCommandDef> {
        let mut cmds: Vec<_> = self.commands.values().collect();
        cmds.sort_by(|a, b| a.name.cmp(&b.name));
        cmds
    }

    pub fn count(&self) -> usize { self.commands.len() }

    /// Expand a user command invocation, substituting <args>, <bang>, <q-args>.
    pub fn expand(&self, name: &str, args: &str, bang: bool) -> Option<String> {
        let def = self.commands.get(name)?;
        let mut result = def.replacement.clone();
        result = result.replace("<args>", args);
        result = result.replace("<q-args>", &format!("\"{}\"", args));
        result = result.replace("<bang>", if bang { "!" } else { "" });
        Some(result)
    }
}

/// Parse a :command definition from arguments.
pub fn parse_command_def(args: &str) -> Result<UserCommandDef, String> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    if parts.is_empty() { return Err("Missing command name".into()); }
    let name = parts[0].to_string();
    let replacement = parts.get(1).unwrap_or(&"").to_string();
    Ok(UserCommandDef { name, replacement, nargs: NArgs::Any, range_allowed: false,
        bang_allowed: false, bar_allowed: false, complete: None, buffer_local: false })
}

/// Check if a name could be a user command (starts with uppercase).
pub fn is_user_command_name(name: &str) -> bool {
    name.chars().next().map_or(false, |c| c.is_uppercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_def(name: &str, repl: &str) -> UserCommandDef {
        UserCommandDef { name: name.into(), replacement: repl.into(), nargs: NArgs::Any,
            range_allowed: false, bang_allowed: false, bar_allowed: false,
            complete: None, buffer_local: false }
    }

    #[test]
    fn define_and_get() {
        let mut reg = UserCommandRegistry::new();
        reg.define(make_def("Hello", "echo hi")).unwrap();
        assert!(reg.get("Hello").is_some());
    }

    #[test]
    fn define_lowercase_fails() {
        let mut reg = UserCommandRegistry::new();
        assert!(reg.define(make_def("hello", "x")).is_err());
    }

    #[test]
    fn remove_command() {
        let mut reg = UserCommandRegistry::new();
        reg.define(make_def("Foo", "bar")).unwrap();
        assert!(reg.remove("Foo"));
        assert!(reg.get("Foo").is_none());
    }

    #[test]
    fn expand_args() {
        let mut reg = UserCommandRegistry::new();
        reg.define(make_def("Greet", "echo <args>")).unwrap();
        assert_eq!(reg.expand("Greet", "world", false).unwrap(), "echo world");
    }

    #[test]
    fn expand_bang() {
        let mut reg = UserCommandRegistry::new();
        reg.define(make_def("Save", "write<bang>")).unwrap();
        assert_eq!(reg.expand("Save", "", true).unwrap(), "write!");
    }

    #[test]
    fn nargs_validation() {
        assert!(NArgs::Zero.validate_count(0));
        assert!(!NArgs::Zero.validate_count(1));
        assert!(NArgs::AtLeastOne.validate_count(2));
        assert!(!NArgs::AtLeastOne.validate_count(0));
    }

    #[test]
    fn parse_command_basic() {
        let def = parse_command_def("Wq wq").unwrap();
        assert_eq!(def.name, "Wq");
        assert_eq!(def.replacement, "wq");
    }

    #[test]
    fn list_sorted() {
        let mut reg = UserCommandRegistry::new();
        reg.define(make_def("Zzz", "a")).unwrap();
        reg.define(make_def("Aaa", "b")).unwrap();
        let list = reg.list();
        assert_eq!(list[0].name, "Aaa");
    }

    #[test]
    fn is_user_cmd() {
        assert!(is_user_command_name("Hello"));
        assert!(!is_user_command_name("hello"));
    }
}
