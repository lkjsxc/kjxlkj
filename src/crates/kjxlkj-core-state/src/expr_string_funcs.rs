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
