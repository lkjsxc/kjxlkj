//! User functions per /docs/spec/scripting/user-functions.md.
//!
//! Runtime-defined functions callable from commands.

/// A user-defined function.
#[derive(Debug, Clone)]
pub struct UserFunction {
    /// Function name (must start with uppercase).
    pub name: String,
    /// Parameter names.
    pub params: Vec<String>,
    /// Body commands.
    pub body: Vec<String>,
    /// Whether function has variable args (...).
    pub variadic: bool,
    /// Whether function should abort on error.
    pub abort: bool,
}

/// User function registry.
#[derive(Debug, Clone, Default)]
pub struct UserFunctionRegistry {
    /// Registered functions.
    pub functions: Vec<UserFunction>,
}

impl UserFunctionRegistry {
    /// Create empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Define or replace a function.
    pub fn define(&mut self, func: UserFunction) {
        // Remove existing with same name.
        self.functions
            .retain(|f| f.name != func.name);
        self.functions.push(func);
    }

    /// Find a function by name.
    pub fn find(
        &self,
        name: &str,
    ) -> Option<&UserFunction> {
        self.functions
            .iter()
            .find(|f| f.name == name)
    }

    /// Remove a function.
    pub fn remove(&mut self, name: &str) -> bool {
        let len = self.functions.len();
        self.functions
            .retain(|f| f.name != name);
        self.functions.len() < len
    }

    /// List all function names.
    pub fn list(&self) -> Vec<&str> {
        self.functions
            .iter()
            .map(|f| f.name.as_str())
            .collect()
    }
}

/// Parse a `:function` definition block.
pub fn parse_function_def(
    lines: &[&str],
) -> Option<UserFunction> {
    let first = lines.first()?;
    let trimmed = first.trim();
    // function! Name(args)
    let rest = trimmed
        .strip_prefix("function!")
        .or_else(|| trimmed.strip_prefix("function"))?
        .trim();
    let paren = rest.find('(')?;
    let name = rest[..paren].trim().to_string();

    if !name
        .chars()
        .next()
        .map(|c| c.is_uppercase())
        .unwrap_or(false)
    {
        return None;
    }

    let close = rest.find(')')?;
    let params_str = &rest[paren + 1..close];
    let variadic = params_str.contains("...");
    let params: Vec<String> = params_str
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && s != "...")
        .collect();

    let abort = rest[close + 1..]
        .trim()
        .contains("abort");

    // Body: everything except first/last lines.
    let body: Vec<String> = lines[1..]
        .iter()
        .take_while(|l| {
            l.trim() != "endfunction"
                && l.trim() != "endfunction!"
        })
        .map(|l| l.to_string())
        .collect();

    Some(UserFunction {
        name,
        params,
        body,
        variadic,
        abort,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_define_find() {
        let mut reg = UserFunctionRegistry::new();
        reg.define(UserFunction {
            name: "Hello".into(),
            params: vec!["name".into()],
            body: vec!["echo a:name".into()],
            variadic: false,
            abort: false,
        });
        assert!(reg.find("Hello").is_some());
    }

    #[test]
    fn parse_function() {
        let lines = vec![
            "function! Greet(name)",
            "  echo a:name",
            "endfunction",
        ];
        let func = parse_function_def(&lines).unwrap();
        assert_eq!(func.name, "Greet");
        assert_eq!(func.params, vec!["name"]);
        assert_eq!(func.body.len(), 1);
    }

    #[test]
    fn parse_function_abort() {
        let lines = vec![
            "function! Run() abort",
            "  call X()",
            "endfunction",
        ];
        let func = parse_function_def(&lines).unwrap();
        assert!(func.abort);
    }
}
