//! Expression register (=) evaluation.
//! Supports arithmetic, string ops, comparisons, variables.
#![allow(dead_code)]
use std::collections::HashMap;

/// Evaluate a simple expression for the = register.
pub fn eval_expression(expr: &str) -> Result<String, String> {
    eval_expression_with_vars(expr, &HashMap::new())
}

/// Evaluate expression with variable bindings.
#[rustfmt::skip]
pub fn eval_expression_with_vars(
    expr: &str, vars: &HashMap<String, String>,
) -> Result<String, String> {
    let expr = expr.trim();
    if expr.is_empty() { return Ok(String::new()); }
    if let Some(r) = try_builtin_function(expr, vars) { return r; }
    if let Some(rest) = expr.strip_prefix("g:").or_else(|| expr.strip_prefix("b:"))
        .or_else(|| expr.strip_prefix("w:")).or_else(|| expr.strip_prefix("v:"))
    {
        let key = format!("{}{}", &expr[..2], rest);
        return Ok(vars.get(&key).cloned().unwrap_or_default());
    }
    if let Some(parts) = split_concat(expr) {
        let mut r = String::new();
        for p in parts { r.push_str(&eval_expression_with_vars(p.trim(), vars)?); }
        return Ok(r);
    }
    if expr.starts_with('"') && expr.ends_with('"') && expr.len() >= 2 {
        return Ok(expr[1..expr.len() - 1].to_string());
    }
    if expr.starts_with('[') && expr.ends_with(']') {
        return Ok(expr.to_string()); // list literal returned as-is
    }
    if let Some(r) = try_ternary(expr, vars) { return r; }
    if let Some(r) = try_comparison(expr, vars) { return r; }
    eval_arithmetic(expr)
}

/// Split on ` . ` outside quotes.
#[rustfmt::skip]
fn split_concat(expr: &str) -> Option<Vec<&str>> {
    let (mut in_str, bytes) = (false, expr.as_bytes());
    let (mut splits, mut last, mut i) = (Vec::new(), 0usize, 0usize);
    while i < bytes.len() {
        if bytes[i] == b'"' { in_str = !in_str; }
        else if !in_str && i + 3 <= bytes.len() && &bytes[i..i + 3] == b" . " {
            splits.push(&expr[last..i]); last = i + 3; i += 3; continue;
        }
        i += 1;
    }
    if splits.is_empty() { return None; }
    splits.push(&expr[last..]); Some(splits)
}

#[rustfmt::skip]
fn eval_arithmetic(expr: &str) -> Result<String, String> {
    let expr = expr.trim();
    if let Some(pos) = find_op(expr, &['+', '-']) {
        let (l, r) = (pi64(&eval_arithmetic(&expr[..pos])?)?, pi64(&eval_arithmetic(&expr[pos + 1..])?)?);
        return Ok(format!("{}", if expr.as_bytes()[pos] == b'+' { l + r } else { l - r }));
    }
    if let Some(pos) = find_op(expr, &['*', '/', '%']) {
        let (l, r) = (pi64(&eval_arithmetic(&expr[..pos])?)?, pi64(&eval_arithmetic(&expr[pos + 1..])?)?);
        if r == 0 && expr.as_bytes()[pos] != b'*' { return Err("Division by zero".into()); }
        let res = match expr.as_bytes()[pos] { b'*' => l * r, b'/' => l / r, _ => l % r };
        return Ok(format!("{res}"));
    }
    if expr.starts_with('(') && expr.ends_with(')') { return eval_arithmetic(&expr[1..expr.len() - 1]); }
    pi64(expr).map(|_| expr.to_string())
}
fn pi64(s: &str) -> Result<i64, String> { s.trim().parse().map_err(|_| format!("Invalid number: {s}")) }

/// Find rightmost op at top-level (not inside parens).
#[rustfmt::skip]
fn find_op(expr: &str, ops: &[char]) -> Option<usize> {
    let (bytes, mut depth, mut found) = (expr.as_bytes(), 0i32, None);
    for (i, &b) in bytes.iter().enumerate() {
        match b { b'(' => depth += 1, b')' => depth -= 1,
            _ if depth == 0 && i > 0 && ops.contains(&(b as char)) => found = Some(i), _ => {} }
    }
    found
}

/// Try ternary: `cond ? then : else`.
#[rustfmt::skip]
fn try_ternary(expr: &str, vars: &HashMap<String, String>) -> Option<Result<String, String>> {
    let qpos = find_top_level_char(expr, '?')?;
    let rest = &expr[qpos + 1..];
    let cpos = find_top_level_char(rest, ':')?;
    let cond = expr[..qpos].trim();
    let then_part = rest[..cpos].trim();
    let else_part = rest[cpos + 1..].trim();
    let cv = match eval_expression_with_vars(cond, vars) { Ok(v) => v, Err(e) => return Some(Err(e)) };
    let truthy = cv != "0" && !cv.is_empty();
    Some(eval_expression_with_vars(if truthy { then_part } else { else_part }, vars))
}

/// Find top-level char (not inside parens/quotes).
#[rustfmt::skip]
fn find_top_level_char(expr: &str, target: char) -> Option<usize> {
    let (bytes, mut depth, mut in_str) = (expr.as_bytes(), 0i32, false);
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'"' { in_str = !in_str; continue; }
        if in_str { continue; }
        match b { b'(' => depth += 1, b')' => depth -= 1, _ => {} }
        if depth == 0 && b == target as u8 { return Some(i); }
    }
    None
}

/// Try to evaluate a comparison expression (==, !=, <=, >=, <, >).
/// Returns "1" for true, "0" for false.
#[rustfmt::skip]
fn try_comparison(
    expr: &str, vars: &HashMap<String, String>,
) -> Option<Result<String, String>> {
    // Order matters: check two-char ops before single-char.
    for &op in &["==", "!=", "<=", ">=", "<", ">"] {
        if let Some(pos) = find_comparison_op(expr, op) {
            let left = match eval_expression_with_vars(expr[..pos].trim(), vars) {
                Ok(v) => v, Err(e) => return Some(Err(e)),
            };
            let right = match eval_expression_with_vars(expr[pos + op.len()..].trim(), vars) {
                Ok(v) => v, Err(e) => return Some(Err(e)),
            };
            let result = match (left.parse::<i64>(), right.parse::<i64>()) {
                (Ok(l), Ok(r)) => match op {
                    "==" => l == r, "!=" => l != r, "<" => l < r,
                    ">" => l > r, "<=" => l <= r, ">=" => l >= r, _ => false,
                },
                _ => match op {
                    "==" => left == right, "!=" => left != right, "<" => left < right,
                    ">" => left > right, "<=" => left <= right, ">=" => left >= right, _ => false,
                },
            };
            return Some(Ok(if result { "1" } else { "0" }.to_string()));
        }
    }
    None
}

/// Find comparison op at top-level (outside parens/quotes).
#[rustfmt::skip]
fn find_comparison_op(expr: &str, op: &str) -> Option<usize> {
    let (bytes, ob, mut depth, mut in_str) = (expr.as_bytes(), op.as_bytes(), 0i32, false);
    for i in 0..bytes.len() {
        if bytes[i] == b'"' { in_str = !in_str; continue; }
        if in_str { continue; }
        match bytes[i] { b'(' => depth += 1, b')' => depth -= 1, _ => {} }
        if depth == 0 && i > 0 && i + ob.len() <= bytes.len() && &bytes[i..i + ob.len()] == ob { return Some(i); }
    }
    None
}
#[rustfmt::skip]
fn try_builtin_function(
    expr: &str, vars: &HashMap<String, String>,
) -> Option<Result<String, String>> {
    let paren = expr.find('(')?;
    if !expr.ends_with(')') { return None; }
    let name = expr[..paren].trim();
    let arg = expr[paren + 1..expr.len() - 1].trim();
    match name {
        "strlen" => {
            let val = match eval_expression_with_vars(arg, vars) {
                Ok(v) => v, Err(e) => return Some(Err(e)),
            };
            Some(Ok(format!("{}", val.len())))
        }
        "len" => {
            let val = match eval_expression_with_vars(arg, vars) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            if val.starts_with('[') && val.ends_with(']') {
                let inner = val[1..val.len()-1].trim();
                let count = if inner.is_empty() { 0 } else { inner.split(',').count() };
                Some(Ok(format!("{count}")))
            } else { Some(Ok(format!("{}", val.len()))) }
        }
        "line" if arg == "\".\"" || arg == "'.'" || arg == "." =>
            Some(Ok(vars.get("v:lnum").cloned().unwrap_or_else(|| "1".into()))),
        "col" if arg == "\".\"" || arg == "'.'" || arg == "." =>
            Some(Ok(vars.get("v:col").cloned().unwrap_or_else(|| "1".into()))),
        "line" | "col" => Some(Ok("0".into())),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_basic_arithmetic() { assert_eq!(eval_expression("2+3").unwrap(), "5"); assert_eq!(eval_expression("10-4").unwrap(), "6"); assert_eq!(eval_expression("3*4").unwrap(), "12"); }
    #[test] fn test_strings() { assert_eq!(eval_expression("\"hello\"").unwrap(), "hello"); assert_eq!(eval_expression("\"hello\" . \" world\"").unwrap(), "hello world"); }
}
