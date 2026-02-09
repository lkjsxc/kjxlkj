//! User-defined Vimscript function registry.
//! Supports `function!` / `endfunction` definitions with
//! named parameters, stored body, and invocation.

use std::collections::HashMap;

/// A user-defined function.
#[derive(Debug, Clone)]
pub struct UserFunction {
    /// Function name (e.g., "MyFunc").
    pub name: String,
    /// Parameter names.
    pub params: Vec<String>,
    /// Body lines (ex commands).
    pub body: Vec<String>,
}

/// Registry of user-defined functions.
#[derive(Debug, Default)]
pub struct FunctionRegistry {
    functions: HashMap<String, UserFunction>,
}

impl FunctionRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Define or overwrite a function.
    pub fn define(&mut self, func: UserFunction) {
        self.functions.insert(func.name.clone(), func);
    }

    /// Look up a function by name.
    pub fn get(&self, name: &str) -> Option<&UserFunction> {
        self.functions.get(name)
    }

    /// Remove a function.
    pub fn remove(&mut self, name: &str) -> bool {
        self.functions.remove(name).is_some()
    }

    /// List all functions.
    pub fn list(&self) -> Vec<&UserFunction> {
        let mut v: Vec<_> = self.functions.values().collect();
        v.sort_by_key(|f| &f.name);
        v
    }

    pub fn len(&self) -> usize {
        self.functions.len()
    }
    pub fn is_empty(&self) -> bool {
        self.functions.is_empty()
    }
}

/// Parse a `function! Name(args)` header line.
/// Returns (name, params) on success.
pub fn parse_function_header(line: &str) -> Result<(String, Vec<String>), String> {
    let line = line.trim();
    let rest = line
        .strip_prefix("function!")
        .or_else(|| line.strip_prefix("function"))
        .ok_or_else(|| "Expected 'function' keyword".to_string())?
        .trim();
    let paren = rest
        .find('(')
        .ok_or_else(|| "Expected '(' after function name".to_string())?;
    let name = rest[..paren].trim().to_string();
    if name.is_empty() {
        return Err("Function name required".to_string());
    }
    let close = rest
        .find(')')
        .ok_or_else(|| "Expected ')' after parameters".to_string())?;
    let param_str = rest[paren + 1..close].trim();
    let params = if param_str.is_empty() {
        Vec::new()
    } else {
        param_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    };
    Ok((name, params))
}
