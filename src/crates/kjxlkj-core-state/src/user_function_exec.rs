//! User function execution — deterministic function calls with argument binding and return values.

use std::collections::HashMap;
use crate::scripting::UserFunction;

/// Result of a function call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncResult {
    Value(String),
    Void,
    Error(String),
}

/// A simple evaluation context for function bodies.
#[derive(Debug, Clone)]
pub struct FuncContext {
    pub locals: HashMap<String, String>,
    pub return_value: Option<String>,
    pub aborted: bool,
}

impl FuncContext {
    pub fn new() -> Self { Self { locals: HashMap::new(), return_value: None, aborted: false } }

    pub fn bind_args(&mut self, params: &[String], args: &[String]) {
        for (i, param) in params.iter().enumerate() {
            let val = args.get(i).cloned().unwrap_or_default();
            self.locals.insert(param.clone(), val);
        }
        // a:0 = extra arg count, a:000 = varargs list (simplified)
        let extra = args.len().saturating_sub(params.len());
        self.locals.insert("a:0".into(), extra.to_string());
    }

    pub fn set_return(&mut self, val: &str) {
        self.return_value = Some(val.into());
    }

    pub fn resolve_var(&self, name: &str) -> Option<&str> {
        self.locals.get(name).map(|s| s.as_str())
    }
}

/// Execute a user function body line-by-line (simplified interpreter).
pub fn execute_function(func: &UserFunction, args: &[String]) -> FuncResult {
    let mut ctx = FuncContext::new();
    ctx.bind_args(&func.params, args);
    for line in &func.body {
        let trimmed = line.trim();
        if trimmed.starts_with("return ") {
            let expr = &trimmed[7..];
            let resolved = resolve_expression(expr, &ctx);
            ctx.set_return(&resolved);
            break;
        }
        if trimmed.starts_with("let ") {
            if let Some((name, val)) = parse_let(trimmed) {
                let resolved = resolve_expression(&val, &ctx);
                ctx.locals.insert(name, resolved);
            }
        }
        if func.is_abort && ctx.aborted { break; }
    }
    match ctx.return_value {
        Some(v) => FuncResult::Value(v),
        None => FuncResult::Void,
    }
}

/// Parse a `let var = expr` statement.
fn parse_let(line: &str) -> Option<(String, String)> {
    let rest = line.strip_prefix("let ")?;
    let eq = rest.find('=')?;
    let name = rest[..eq].trim().to_string();
    let val = rest[eq + 1..].trim().to_string();
    Some((name, val))
}

/// Resolve a simple expression: variable lookup or literal.
fn resolve_expression(expr: &str, ctx: &FuncContext) -> String {
    let expr = expr.trim();
    if let Some(val) = ctx.resolve_var(expr) { return val.to_string(); }
    // Simple string concat with `.`
    if expr.contains(" . ") {
        return expr.split(" . ")
            .map(|part| {
                let p = part.trim().trim_matches('"').trim_matches('\'');
                ctx.resolve_var(p).unwrap_or(p).to_string()
            })
            .collect::<Vec<_>>()
            .join("");
    }
    expr.trim_matches('"').trim_matches('\'').into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn func(name: &str, params: Vec<&str>, body: Vec<&str>) -> UserFunction {
        UserFunction {
            name: name.into(), params: params.into_iter().map(Into::into).collect(),
            body: body.into_iter().map(Into::into).collect(), is_dict: false, is_abort: false,
        }
    }

    #[test]
    fn simple_return() {
        let f = func("Greet", vec!["name"], vec!["return name"]);
        let r = execute_function(&f, &["Alice".into()]);
        assert_eq!(r, FuncResult::Value("Alice".into()));
    }

    #[test]
    fn no_return_gives_void() {
        let f = func("Noop", vec![], vec!["let x = 1"]);
        assert_eq!(execute_function(&f, &[]), FuncResult::Void);
    }

    #[test]
    fn let_and_return() {
        let f = func("Add", vec!["a"], vec!["let result = a", "return result"]);
        let r = execute_function(&f, &["42".into()]);
        assert_eq!(r, FuncResult::Value("42".into()));
    }

    #[test]
    fn string_concat() {
        let f = func("Greet", vec!["name"], vec!["return \"hello \" . name"]);
        let r = execute_function(&f, &["world".into()]);
        assert_eq!(r, FuncResult::Value("hello world".into()));
    }

    #[test]
    fn missing_arg_defaults_empty() {
        let f = func("Test", vec!["a", "b"], vec!["return b"]);
        let r = execute_function(&f, &["only_one".into()]);
        assert_eq!(r, FuncResult::Value("".into()));
    }

    #[test]
    fn extra_args_count() {
        let f = func("Test", vec!["a"], vec!["return a:0"]);
        let r = execute_function(&f, &["x".into(), "y".into(), "z".into()]);
        assert_eq!(r, FuncResult::Value("2".into()));
    }

    #[test]
    fn parse_let_stmt() {
        let (name, val) = parse_let("let x = hello").unwrap();
        assert_eq!(name, "x");
        assert_eq!(val, "hello");
    }
}
