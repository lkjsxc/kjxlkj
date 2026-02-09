//! String functions for expression evaluator: match(), substitute(), etc.
use std::collections::HashMap;

/// Handle match(str, pattern) — returns byte index of match or -1.
pub fn expr_match(arg: &str, vars: &HashMap<String, String>) -> Result<String, String> {
    let (sa, pa) = match super::expr_eval::split_two_args_pub(arg) {
        Some(p) => p,
        None => return Err("match() requires 2 args".into()),
    };
    let s = super::expr_eval::eval_expression_with_vars(sa.trim(), vars)?;
    let pat_raw = super::expr_eval::eval_expression_with_vars(pa.trim(), vars)?;
    match regex::Regex::new(&pat_raw) {
        Ok(re) => Ok(re.find(&s).map(|m| m.start() as i64).unwrap_or(-1).to_string()),
        Err(_) => Ok("-1".to_string()),
    }
}

/// Handle substitute(str, pat, rep, flags) — regex substitution.
pub fn expr_substitute(arg: &str, vars: &HashMap<String, String>) -> Result<String, String> {
    // Parse 4 args: str, pat, rep, flags.
    let (sa, rest) = match super::expr_eval::split_two_args_pub(arg) {
        Some(p) => p,
        None => return Err("substitute() requires 4 args".into()),
    };
    let (pa, rest2) = match super::expr_eval::split_two_args_pub(rest.trim()) {
        Some(p) => p,
        None => return Err("substitute() requires 4 args".into()),
    };
    let (ra, fa) = match super::expr_eval::split_two_args_pub(rest2.trim()) {
        Some(p) => p,
        None => return Err("substitute() requires 4 args".into()),
    };
    let s = super::expr_eval::eval_expression_with_vars(sa.trim(), vars)?;
    let pat = super::expr_eval::eval_expression_with_vars(pa.trim(), vars)?;
    let rep = super::expr_eval::eval_expression_with_vars(ra.trim(), vars)?;
    let flags = super::expr_eval::eval_expression_with_vars(fa.trim(), vars)?;
    let global = flags.contains('g');
    match regex::Regex::new(&pat) {
        Ok(re) => {
            let result = if global {
                re.replace_all(&s, rep.as_str()).to_string()
            } else {
                re.replace(&s, rep.as_str()).to_string()
            };
            Ok(result)
        }
        Err(e) => Err(format!("E383: Invalid pattern: {e}")),
    }
}

/// Handle tr(string, fromchars, tochars) — character-by-character transliteration.
pub fn expr_tr(arg: &str, vars: &HashMap<String, String>) -> Result<String, String> {
    let (sa, rest) = match super::expr_eval::split_two_args_pub(arg) {
        Some(p) => p,
        None => return Err("tr() requires 3 args".into()),
    };
    let (fa, ta) = match super::expr_eval::split_two_args_pub(rest.trim()) {
        Some(p) => p,
        None => return Err("tr() requires 3 args".into()),
    };
    let s = super::expr_eval::eval_expression_with_vars(sa.trim(), vars)?;
    let from: Vec<char> = super::expr_eval::eval_expression_with_vars(fa.trim(), vars)?.chars().collect();
    let to: Vec<char> = super::expr_eval::eval_expression_with_vars(ta.trim(), vars)?.chars().collect();
    let result: String = s.chars().map(|c| {
        from.iter().position(|&f| f == c).and_then(|i| to.get(i).copied()).unwrap_or(c)
    }).collect();
    Ok(result)
}

/// Handle escape(string, chars) — prepend backslash before each occurrence of chars.
pub fn expr_escape(arg: &str, vars: &HashMap<String, String>) -> Result<String, String> {
    let (sa, ca) = match super::expr_eval::split_two_args_pub(arg) {
        Some(p) => p,
        None => return Err("escape() requires 2 args".into()),
    };
    let s = super::expr_eval::eval_expression_with_vars(sa.trim(), vars)?;
    let esc_chars: Vec<char> = super::expr_eval::eval_expression_with_vars(ca.trim(), vars)?.chars().collect();
    let mut result = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        if esc_chars.contains(&c) { result.push('\\'); }
        result.push(c);
    }
    Ok(result)
}

/// Handle printf(fmt, arg1, arg2, ...) — simple %s/%d format.
#[rustfmt::skip]
pub fn expr_printf(arg: &str, vars: &HashMap<String, String>) -> Result<String, String> {
    let (fa, rest) = match super::expr_eval::split_two_args_pub(arg) {
        Some(p) => p, None => return super::expr_eval::eval_expression_with_vars(arg.trim(), vars),
    };
    let fmt = super::expr_eval::eval_expression_with_vars(fa.trim(), vars)?;
    let mut args: Vec<String> = Vec::new();
    let mut remaining = rest.trim();
    loop {
        if remaining.is_empty() { break; }
        match super::expr_eval::split_two_args_pub(remaining) {
            Some((a, r)) => { args.push(super::expr_eval::eval_expression_with_vars(a.trim(), vars)?); remaining = r.trim(); }
            None => { args.push(super::expr_eval::eval_expression_with_vars(remaining, vars)?); break; }
        }
    }
    let mut result = String::new();
    let mut ai = 0usize;
    let bytes = fmt.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 1 < bytes.len() {
            match bytes[i + 1] {
                b's' => { result.push_str(args.get(ai).map(|s| s.as_str()).unwrap_or("")); ai += 1; i += 2; }
                b'd' => { let v = args.get(ai).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0); result.push_str(&v.to_string()); ai += 1; i += 2; }
                b'%' => { result.push('%'); i += 2; }
                _ => { result.push('%'); i += 1; }
            }
        } else { result.push(bytes[i] as char); i += 1; }
    }
    Ok(result)
}

/// Handle split(string, pattern) — split string by pattern into a list.
pub fn expr_split(arg: &str, vars: &HashMap<String, String>) -> Result<String, String> {
    let (sa, pa) = match super::expr_eval::split_two_args_pub(arg) {
        Some(p) => p,
        None => { let s = super::expr_eval::eval_expression_with_vars(arg.trim(), vars)?; let items: Vec<&str> = s.split_whitespace().collect(); return Ok(format!("[{}]", items.iter().map(|i| format!("\"{}\"", i)).collect::<Vec<_>>().join(","))); }
    };
    let s = super::expr_eval::eval_expression_with_vars(sa.trim(), vars)?;
    let pat = super::expr_eval::eval_expression_with_vars(pa.trim(), vars)?;
    let items: Vec<&str> = s.split(&pat).collect();
    Ok(format!("[{}]", items.iter().map(|i| format!("\"{}\"", i)).collect::<Vec<_>>().join(",")))
}

/// Handle join(list, sep) — join list items with separator.
pub fn expr_join(arg: &str, vars: &HashMap<String, String>) -> Result<String, String> {
    let (la, sa) = match super::expr_eval::split_two_args_pub(arg) {
        Some(p) => p,
        None => return Err("join() requires 2 args".into()),
    };
    let list_val = super::expr_eval::eval_expression_with_vars(la.trim(), vars)?;
    let sep = super::expr_eval::eval_expression_with_vars(sa.trim(), vars)?;
    let inner = list_val.trim().strip_prefix('[').and_then(|s| s.strip_suffix(']')).unwrap_or(&list_val);
    if inner.is_empty() { return Ok(String::new()); }
    let items: Vec<String> = inner.split(',').map(|s| s.trim().trim_matches('"').to_string()).collect();
    Ok(items.join(&sep))
}
