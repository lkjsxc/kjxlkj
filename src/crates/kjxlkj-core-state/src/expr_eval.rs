//! Expression register (=) evaluation: arithmetic, string, comparison, lists, dicts.
#![allow(dead_code)]
use std::collections::HashMap;
pub fn eval_expression(expr: &str) -> Result<String, String> { eval_expression_with_vars(expr, &HashMap::new()) }
#[rustfmt::skip]
pub fn eval_expression_with_vars(expr: &str, vars: &HashMap<String, String>) -> Result<String, String> {
    let expr = expr.trim();
    if expr.is_empty() { return Ok(String::new()); }
    if let Some(r) = try_builtin_function(expr, vars) { return r; }
    if let Some(rest) = expr.strip_prefix("g:").or_else(|| expr.strip_prefix("b:"))
        .or_else(|| expr.strip_prefix("w:")).or_else(|| expr.strip_prefix("v:")).or_else(|| expr.strip_prefix("s:"))
    { return Ok(vars.get(&format!("{}{}", &expr[..2], rest)).cloned().unwrap_or_default()); }
    if let Some(b) = expr.find("[\"") { if expr.ends_with("\"]") { return Ok(extract_dict_value(&eval_expression_with_vars(&expr[..b], vars)?, &expr[b+2..expr.len()-2])); } }
    if expr.starts_with("function(\"") && expr.ends_with("\")") { return Ok(expr[10..expr.len()-2].to_string()); }
    if let Some(parts) = split_concat(expr) { let mut r = String::new(); for p in parts { r.push_str(&eval_expression_with_vars(p.trim(), vars)?); } return Ok(r); }
    if expr.starts_with('"') && expr.ends_with('"') && expr.len() >= 2 { return Ok(expr[1..expr.len()-1].to_string()); }
    if expr.starts_with('[') && expr.ends_with(']') { return Ok(expr.to_string()); }
    if expr.starts_with('{') && expr.ends_with('}') { return Ok(expr.to_string()); }
    if let Some(r) = try_ternary(expr, vars) { return r; }
    if let Some(r) = try_comparison(expr, vars) { return r; }
    if let Some(v) = vars.get(expr) { return Ok(v.clone()); }
    eval_arithmetic(expr)
}

#[rustfmt::skip]
fn split_concat(expr: &str) -> Option<Vec<&str>> {
    let (mut in_str, bytes) = (false, expr.as_bytes());
    let (mut splits, mut last, mut i) = (Vec::new(), 0usize, 0usize);
    while i < bytes.len() {
        if bytes[i] == b'"' { in_str = !in_str; } else if !in_str && i+3 <= bytes.len() && &bytes[i..i+3] == b" . " { splits.push(&expr[last..i]); last = i+3; i += 3; continue; }
        i += 1;
    }
    if splits.is_empty() { None } else { splits.push(&expr[last..]); Some(splits) }
}
#[rustfmt::skip]
fn eval_arithmetic(expr: &str) -> Result<String, String> {
    let expr = expr.trim();
    if let Some(pos) = find_op(expr, &['+', '-']) {
        let (l, r) = (&eval_arithmetic(&expr[..pos])?, &eval_arithmetic(&expr[pos + 1..])?);
        return arith_op(l, r, expr.as_bytes()[pos]);
    }
    if let Some(pos) = find_op(expr, &['*', '/', '%']) {
        let (l, r) = (&eval_arithmetic(&expr[..pos])?, &eval_arithmetic(&expr[pos + 1..])?);
        return arith_op(l, r, expr.as_bytes()[pos]);
    }
    if expr.starts_with('(') && expr.ends_with(')') { return eval_arithmetic(&expr[1..expr.len() - 1]); }
    if let Some(r) = try_builtin_function(expr, &std::collections::HashMap::new()) { return r; }
    parse_number(expr).map(|_| expr.trim().to_string())
}
#[rustfmt::skip]
fn arith_op(l: &str, r: &str, op: u8) -> Result<String, String> {
    if let (Ok(li), Ok(ri)) = (pi64(l), pi64(r)) {
        if ri == 0 && (op == b'/' || op == b'%') { return Err("Division by zero".into()); }
        return Ok(format!("{}", match op { b'+' => li+ri, b'-' => li-ri, b'*' => li*ri, b'/' => li/ri, _ => li%ri }));
    }
    let (lf, rf) = (parse_number(l)?, parse_number(r)?);
    if rf == 0.0 && (op == b'/' || op == b'%') { return Err("Division by zero".into()); }
    Ok(format!("{}", match op { b'+' => lf+rf, b'-' => lf-rf, b'*' => lf*rf, b'/' => lf/rf, _ => lf%rf }))
}
fn pi64(s: &str) -> Result<i64, std::num::ParseIntError> { s.trim().parse() }
fn parse_number(s: &str) -> Result<f64, String> { s.trim().parse::<f64>().map_err(|e| e.to_string()) }
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
#[rustfmt::skip]
fn try_ternary(expr: &str, vars: &HashMap<String, String>) -> Option<Result<String, String>> {
    let qpos = find_top_level_char(expr, '?')?;
    let rest = &expr[qpos + 1..]; let cpos = find_top_level_char(rest, ':')?;
    let cv = match eval_expression_with_vars(expr[..qpos].trim(), vars) { Ok(v) => v, Err(e) => return Some(Err(e)) };
    Some(eval_expression_with_vars(if cv != "0" && !cv.is_empty() { rest[..cpos].trim() } else { rest[cpos+1..].trim() }, vars))
}
#[rustfmt::skip]
fn find_top_level_char(expr: &str, target: char) -> Option<usize> {
    let (bytes, mut depth, mut in_str) = (expr.as_bytes(), 0i32, false);
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'"' { in_str = !in_str; continue; } else if in_str { continue; }
        match b { b'(' => depth += 1, b')' => depth -= 1, _ => {} }
        if depth == 0 && b == target as u8 { return Some(i); }
    } None
}
#[rustfmt::skip]
fn try_comparison(expr: &str, vars: &HashMap<String, String>) -> Option<Result<String, String>> {
    for &op in &["==", "!=", "<=", ">=", "<", ">"] {
        if let Some(pos) = find_comparison_op(expr, op) {
            let left = match eval_expression_with_vars(expr[..pos].trim(), vars) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let right = match eval_expression_with_vars(expr[pos+op.len()..].trim(), vars) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            let r = match (left.parse::<i64>(), right.parse::<i64>()) {
                (Ok(l), Ok(r)) => match op { "==" => l==r, "!=" => l!=r, "<" => l<r, ">" => l>r, "<=" => l<=r, _ => l>=r },
                _ => match op { "==" => left==right, "!=" => left!=right, "<" => left<right, ">" => left>right, "<=" => left<=right, _ => left>=right },
            };
            return Some(Ok(if r { "1" } else { "0" }.into()));
        }
    } None
}
#[rustfmt::skip]
fn find_comparison_op(expr: &str, op: &str) -> Option<usize> {
    let (bytes, ob, mut depth, mut in_str) = (expr.as_bytes(), op.as_bytes(), 0i32, false);
    for i in 0..bytes.len() {
        if bytes[i] == b'"' { in_str = !in_str; continue; } else if in_str { continue; }
        match bytes[i] { b'(' => depth += 1, b')' => depth -= 1, _ => {} }
        if depth == 0 && i > 0 && i+ob.len() <= bytes.len() && &bytes[i..i+ob.len()] == ob { return Some(i); }
    } None
}
#[rustfmt::skip]
fn try_builtin_function(expr: &str, vars: &HashMap<String, String>) -> Option<Result<String, String>> {
    let paren = expr.find('(')?;
    if !expr.ends_with(')') { return None; }
    let (name, arg) = (expr[..paren].trim(), expr[paren+1..expr.len()-1].trim());
    match name {
        "strlen" => { let v = match eval_expression_with_vars(arg, vars) { Ok(v) => v, Err(e) => return Some(Err(e)) }; Some(Ok(format!("{}", v.len()))) }
        "len" => {
            let v = match eval_expression_with_vars(arg, vars) { Ok(v) => v, Err(e) => return Some(Err(e)) };
            if v.starts_with('[') && v.ends_with(']') { let inner = v[1..v.len()-1].trim(); Some(Ok(format!("{}", if inner.is_empty() { 0 } else { inner.split(',').count() }))) }
            else { Some(Ok(format!("{}", v.len()))) }
        }
        "line" if arg == "\".\"" || arg == "'.'" || arg == "." => Some(Ok(vars.get("v:lnum").cloned().unwrap_or_else(|| "1".into()))),
        "col" if arg == "\".\"" || arg == "'.'" || arg == "." => Some(Ok(vars.get("v:col").cloned().unwrap_or_else(|| "1".into()))),
        "line" | "col" => Some(Ok("0".into())),
        "type" => { let v = match eval_expression_with_vars(arg, vars) { Ok(v) => v, Err(e) => return Some(Err(e)) }; Some(Ok((if v.starts_with('[') { "3" } else if v.starts_with('{') { "4" } else if v.parse::<i64>().is_ok() { "0" } else { "1" }).into())) }
        "has_key" => {
            let (da, ka) = match split_two_args(arg) { Some(p) => p, None => return Some(Err("has_key() requires 2 args".into())) };
            let d = match eval_expression_with_vars(da.trim(), vars) { Ok(v) => v, Err(e) => return Some(Err(e)) }; let k = ka.trim().trim_matches('"');
            Some(Ok(if d.contains(&format!("\"{}\":", k)) || d.contains(&format!("\"{}\" :", k)) { "1" } else { "0" }.into()))
        }
        "function" => Some(Ok(arg.trim().trim_matches('"').to_string())),
        "keys" => { let v = match eval_expression_with_vars(arg, vars) { Ok(v) => v, Err(e) => return Some(Err(e)) }; Some(Ok(extract_dict_keys(&v))) }
        "values" => { let v = match eval_expression_with_vars(arg, vars) { Ok(v) => v, Err(e) => return Some(Err(e)) }; Some(Ok(extract_dict_values(&v))) }
        "map" => Some(Ok(list_map_filter(arg, vars, true))),
        "filter" => Some(Ok(list_map_filter(arg, vars, false))),
        "extend" => Some(Ok(list_extend(arg, vars))),
        "match" => Some(crate::expr_string_funcs::expr_match(arg, vars)),
        "substitute" => Some(crate::expr_string_funcs::expr_substitute(arg, vars)),
        "and" | "or" | "xor" | "lshift" | "rshift" => { let (la, ra) = match split_two_args(arg) { Some(p) => p, None => return Some(Err(format!("{name}() requires 2 args"))) };
            let (l, r) = (eval_expression_with_vars(la.trim(), vars).ok()?.parse::<i64>().ok()?, eval_expression_with_vars(ra.trim(), vars).ok()?.parse::<i64>().ok()?);
            Some(Ok(format!("{}", match name { "and" => l & r, "or" => l | r, "xor" => l ^ r, "lshift" => l << r, _ => l >> r })))
        }
        _ => None,
    }
}
fn extract_dict_value(dict: &str, key: &str) -> String { // Extract a value from a JSON-ish dict string by key.
    let needle = format!("\"{}\":", key);
    if let Some(pos) = dict.find(&needle) {
        let after = dict[pos + needle.len()..].trim_start();
        if let Some(inner) = after.strip_prefix('"') { inner.split('"').next().unwrap_or("").to_string() }
        else { after.split(&[',', '}'][..]).next().unwrap_or("").trim().to_string() }
    } else { String::new() }
}
fn extract_dict_keys(dict: &str) -> String { // Extract all keys from a JSON-ish dict as a list string.
    let inner = dict.trim().strip_prefix('{').and_then(|s| s.strip_suffix('}')).unwrap_or("");
    let keys: Vec<String> = inner.split(',').filter_map(|pair| { let k = pair.split(':').next()?.trim().trim_matches('"'); if k.is_empty() { None } else { Some(format!("\"{}\"", k)) } }).collect();
    format!("[{}]", keys.join(","))
}
fn extract_dict_values(dict: &str) -> String { // Extract all values from a JSON-ish dict as a list string.
    let inner = dict.trim().strip_prefix('{').and_then(|s| s.strip_suffix('}')).unwrap_or("");
    let vals: Vec<&str> = inner.split(',').filter_map(|pair| { let kv: Vec<&str> = pair.splitn(2, ':').collect(); if kv.len() < 2 { None } else { Some(kv[1].trim()) } }).collect();
    format!("[{}]", vals.join(","))
}
pub fn split_two_args_pub(arg: &str) -> Option<(&str, &str)> { split_two_args(arg) } // public wrapper
fn split_two_args(arg: &str) -> Option<(&str, &str)> {
    let (bytes, mut depth) = (arg.as_bytes(), 0i32);
    for (i, &b) in bytes.iter().enumerate() {
        match b { b'[' | b'{' | b'(' => depth += 1, b']' | b'}' | b')' => depth -= 1, b',' if depth == 0 => return Some((&arg[..i], &arg[i+1..])), _ => {} }
    }
    None
}
#[rustfmt::skip] // map(list, expr) applies expr; filter(list, expr) keeps items where expr != "0"
fn list_map_filter(arg: &str, vars: &HashMap<String, String>, is_map: bool) -> String {
    let (la, ea) = match split_two_args(arg) { Some(p) => p, None => return "[]".into() };
    let list_str = eval_expression_with_vars(la.trim(), vars).unwrap_or_default();
    let expr = ea.trim().trim_matches('"');
    let inner = list_str.trim().strip_prefix('[').and_then(|s| s.strip_suffix(']')).unwrap_or("");
    if inner.is_empty() { return "[]".into(); }
    let result: Vec<String> = inner.split(',').map(|s| s.trim()).filter_map(|item| {
        let mut local = vars.clone(); local.insert("v:val".into(), item.trim_matches('"').to_string());
        let val = eval_expression_with_vars(expr, &local).unwrap_or_default();
        if is_map { Some(val) } else if val != "0" && !val.is_empty() { Some(item.to_string()) } else { None }
    }).collect();
    format!("[{}]", result.join(","))
}
fn list_extend(arg: &str, vars: &HashMap<String, String>) -> String { // extend(list1, list2)
    let (la, lb) = match split_two_args(arg) { Some(p) => p, None => return "[]".into() };
    let (a, b) = (eval_expression_with_vars(la.trim(), vars).unwrap_or_default(), eval_expression_with_vars(lb.trim(), vars).unwrap_or_default());
    let (ia, ib) = (a.trim().strip_prefix('[').and_then(|s| s.strip_suffix(']')).unwrap_or(""), b.trim().strip_prefix('[').and_then(|s| s.strip_suffix(']')).unwrap_or(""));
    let items: Vec<&str> = [ia, ib].iter().filter(|s| !s.is_empty()).flat_map(|s| s.split(',').map(|x| x.trim())).collect();
    format!("[{}]", items.join(","))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_basic_arithmetic() { assert_eq!(eval_expression("2+3").unwrap(), "5"); assert_eq!(eval_expression("10-4").unwrap(), "6"); assert_eq!(eval_expression("3*4").unwrap(), "12"); }
    #[test] fn test_strings() { assert_eq!(eval_expression("\"hello\"").unwrap(), "hello"); assert_eq!(eval_expression("\"hello\" . \" world\"").unwrap(), "hello world"); }
}
