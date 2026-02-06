/// User-defined functions — :function, :call, function registry.

use std::collections::HashMap;

/// A user-defined function.
#[derive(Debug, Clone, PartialEq)]
pub struct UserFunction {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<String>,
    pub is_script_local: bool,
    pub has_range: bool,
    pub has_abort: bool,
}

impl UserFunction {
    pub fn arity(&self) -> usize { self.params.len() }
    pub fn has_varargs(&self) -> bool { self.params.last().map_or(false, |p| p == "...") }
}

/// Registry of user-defined functions.
#[derive(Debug, Default)]
pub struct FunctionRegistry { functions: HashMap<String, UserFunction> }

impl FunctionRegistry {
    pub fn new() -> Self { Self::default() }

    pub fn define(&mut self, func: UserFunction) -> Result<(), String> {
        if func.name.is_empty() { return Err("Empty function name".into()); }
        if !func.name.chars().next().unwrap().is_uppercase() && !func.name.contains(':') {
            return Err("User functions must start with uppercase or use s: prefix".into());
        }
        self.functions.insert(func.name.clone(), func);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&UserFunction> { self.functions.get(name) }
    pub fn remove(&mut self, name: &str) -> bool { self.functions.remove(name).is_some() }
    pub fn count(&self) -> usize { self.functions.len() }

    pub fn list(&self) -> Vec<&UserFunction> {
        let mut fns: Vec<_> = self.functions.values().collect();
        fns.sort_by(|a, b| a.name.cmp(&b.name));
        fns
    }
}

/// Parse a :function definition from lines.
pub fn parse_function(lines: &[&str]) -> Result<UserFunction, String> {
    let first = lines.first().ok_or("Empty function definition")?;
    let header = first.strip_prefix("function").unwrap_or(first).trim();
    let header = header.strip_prefix('!').unwrap_or(header).trim();
    let (name, params_str) = if let Some(paren) = header.find('(') {
        let name = header[..paren].trim().to_string();
        let end = header.find(')').unwrap_or(header.len());
        let params_raw = &header[paren + 1..end];
        let params: Vec<String> = params_raw.split(',').map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty()).collect();
        (name, params)
    } else {
        (header.to_string(), vec![])
    };
    let body: Vec<String> = lines[1..].iter()
        .filter(|l| l.trim() != "endfunction")
        .map(|l| l.to_string()).collect();
    Ok(UserFunction { name, params: params_str, body, is_script_local: false, has_range: false, has_abort: false })
}

/// Evaluate a simple function call string.
pub fn parse_call(input: &str) -> Option<(String, Vec<String>)> {
    let input = input.trim();
    let paren = input.find('(')?;
    let name = input[..paren].trim().to_string();
    let end = input.rfind(')')?;
    let args_str = &input[paren + 1..end];
    let args: Vec<String> = if args_str.trim().is_empty() {
        vec![]
    } else {
        args_str.split(',').map(|s| s.trim().to_string()).collect()
    };
    Some((name, args))
}

/// Check if a function name is script-local.
pub fn is_script_local(name: &str) -> bool {
    name.starts_with("s:") || name.starts_with("<SID>")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_fn(name: &str, params: &[&str]) -> UserFunction {
        UserFunction { name: name.into(), params: params.iter().map(|s| s.to_string()).collect(),
            body: vec![], is_script_local: false, has_range: false, has_abort: false }
    }

    #[test]
    fn define_and_get() {
        let mut reg = FunctionRegistry::new();
        reg.define(make_fn("MyFunc", &["a", "b"])).unwrap();
        let f = reg.get("MyFunc").unwrap();
        assert_eq!(f.arity(), 2);
    }

    #[test]
    fn define_lowercase_fails() {
        let mut reg = FunctionRegistry::new();
        assert!(reg.define(make_fn("myfunc", &[])).is_err());
    }

    #[test]
    fn script_local_allowed() {
        let mut reg = FunctionRegistry::new();
        reg.define(make_fn("s:helper", &[])).unwrap();
        assert!(reg.get("s:helper").is_some());
    }

    #[test]
    fn parse_function_basic() {
        let lines = vec!["function! MyFunc(x, y)", "  echo a:x", "endfunction"];
        let f = parse_function(&lines).unwrap();
        assert_eq!(f.name, "MyFunc");
        assert_eq!(f.params, vec!["x", "y"]);
        assert_eq!(f.body, vec!["  echo a:x"]);
    }

    #[test]
    fn parse_call_basic() {
        let (name, args) = parse_call("MyFunc(1, 2)").unwrap();
        assert_eq!(name, "MyFunc");
        assert_eq!(args, vec!["1", "2"]);
    }

    #[test]
    fn parse_call_no_args() {
        let (name, args) = parse_call("Greet()").unwrap();
        assert_eq!(name, "Greet");
        assert!(args.is_empty());
    }

    #[test]
    fn is_script_local_check() {
        assert!(is_script_local("s:helper"));
        assert!(is_script_local("<SID>run"));
        assert!(!is_script_local("MyFunc"));
    }

    #[test]
    fn has_varargs() {
        let f = make_fn("F", &["a", "..."]);
        assert!(f.has_varargs());
        let f2 = make_fn("G", &["a"]);
        assert!(!f2.has_varargs());
    }

    #[test]
    fn list_sorted() {
        let mut reg = FunctionRegistry::new();
        reg.define(make_fn("Zzz", &[])).unwrap();
        reg.define(make_fn("Aaa", &[])).unwrap();
        let list = reg.list();
        assert_eq!(list[0].name, "Aaa");
    }
}
