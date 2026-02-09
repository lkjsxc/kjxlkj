//! Expression register (=) evaluation.
//!
//! Supports basic arithmetic and string operations
//! for the = register: +, -, *, /, %, string concat.
#![allow(dead_code)]

/// Evaluate a simple expression for the = register.
/// Returns the result as a string, or an error message.
pub fn eval_expression(expr: &str) -> Result<String, String> {
    let expr = expr.trim();
    if expr.is_empty() {
        return Ok(String::new());
    }
    // String concatenation with . (check before single-string literal).
    if let Some(parts) = split_concat(expr) {
        let mut result = String::new();
        for part in parts {
            result.push_str(&eval_expression(part.trim())?);
        }
        return Ok(result);
    }
    // String literal: "..."
    if expr.starts_with('"') && expr.ends_with('"') && expr.len() >= 2 {
        return Ok(expr[1..expr.len() - 1].to_string());
    }
    // Try arithmetic.
    eval_arithmetic(expr)
}

/// Split on ` . ` but only outside quotes.
fn split_concat(expr: &str) -> Option<Vec<&str>> {
    let mut in_str = false;
    let bytes = expr.as_bytes();
    let mut splits = Vec::new();
    let mut last = 0;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'"' {
            in_str = !in_str;
        } else if !in_str && i + 3 <= bytes.len() && &bytes[i..i + 3] == b" . " {
            splits.push(&expr[last..i]);
            last = i + 3;
            i += 3;
            continue;
        }
        i += 1;
    }
    if splits.is_empty() {
        return None;
    }
    splits.push(&expr[last..]);
    Some(splits)
}

fn eval_arithmetic(expr: &str) -> Result<String, String> {
    let expr = expr.trim();
    // Handle addition/subtraction (lowest precedence).
    if let Some(pos) = find_op(expr, &['+', '-']) {
        let left = eval_arithmetic(&expr[..pos])?;
        let right = eval_arithmetic(&expr[pos + 1..])?;
        let l: i64 = left
            .parse()
            .map_err(|_| format!("Invalid number: {left}"))?;
        let r: i64 = right
            .parse()
            .map_err(|_| format!("Invalid number: {right}"))?;
        let op = expr.as_bytes()[pos];
        let res = if op == b'+' { l + r } else { l - r };
        return Ok(format!("{res}"));
    }
    // Handle multiplication/division/modulo.
    if let Some(pos) = find_op(expr, &['*', '/', '%']) {
        let left = eval_arithmetic(&expr[..pos])?;
        let right = eval_arithmetic(&expr[pos + 1..])?;
        let l: i64 = left
            .parse()
            .map_err(|_| format!("Invalid number: {left}"))?;
        let r: i64 = right
            .parse()
            .map_err(|_| format!("Invalid number: {right}"))?;
        let op = expr.as_bytes()[pos];
        if r == 0 && (op == b'/' || op == b'%') {
            return Err("Division by zero".to_string());
        }
        let res = match op {
            b'*' => l * r,
            b'/' => l / r,
            b'%' => l % r,
            _ => unreachable!(),
        };
        return Ok(format!("{res}"));
    }
    // Parenthesized expression.
    if expr.starts_with('(') && expr.ends_with(')') {
        return eval_arithmetic(&expr[1..expr.len() - 1]);
    }
    // Plain number.
    let _: i64 = expr
        .parse()
        .map_err(|_| format!("Invalid expression: {expr}"))?;
    Ok(expr.to_string())
}

/// Find the rightmost operator at top-level (not inside parens).
fn find_op(expr: &str, ops: &[char]) -> Option<usize> {
    let bytes = expr.as_bytes();
    let mut depth = 0i32;
    let mut found = None;
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ if depth == 0 && i > 0 => {
                if ops.contains(&(b as char)) {
                    found = Some(i);
                }
            }
            _ => {}
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        assert_eq!(eval_expression("2+3").unwrap(), "5");
        assert_eq!(eval_expression("10-4").unwrap(), "6");
        assert_eq!(eval_expression("3*4").unwrap(), "12");
        assert_eq!(eval_expression("15/4").unwrap(), "3");
        assert_eq!(eval_expression("17%5").unwrap(), "2");
    }

    #[test]
    fn test_string_literal() {
        assert_eq!(eval_expression("\"hello\"").unwrap(), "hello");
    }

    #[test]
    fn test_string_concat() {
        assert_eq!(
            eval_expression("\"hello\" . \" world\"").unwrap(),
            "hello world"
        );
    }
}
