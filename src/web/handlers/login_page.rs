pub fn render_login_page(username: &str, errors: &[&str]) -> String {
    let escaped_username = escape_html(username);
    let error_block = render_error_block(errors);
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Admin login</title>
</head>
<body>
  <main id="login-page">
    <h1>Admin login</h1>
    <p>Sign in with the admin account created during setup.</p>
    {error_block}
    <form id="login-form" method="post" action="/login">
      <label for="username">Username</label>
      <input id="username" name="username" type="text" autocomplete="username" value="{escaped_username}" />
      <label for="password">Password</label>
      <input id="password" name="password" type="password" autocomplete="current-password" />
      <button type="submit">Sign in</button>
    </form>
  </main>
</body>
</html>"#
    )
}

fn render_error_block(errors: &[&str]) -> String {
    if errors.is_empty() {
        return r#"<section id="login-errors" aria-live="polite"></section>"#.to_owned();
    }

    let items = errors
        .iter()
        .map(|error| format!("<li>{}</li>", escape_html(error)))
        .collect::<String>();
    format!(
        r#"<section id="login-errors" aria-live="polite">
      <p>Unable to sign in:</p>
      <ul>{items}</ul>
    </section>"#
    )
}

fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}
