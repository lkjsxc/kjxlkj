use crate::web::handlers::page_html::escape_html;

pub fn render_login_page(errors: &[&str]) -> String {
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
    <p>Username is fixed as <code>admin</code>. Enter password to continue.</p>
    {error_block}
    <form id="login-form" method="post" action="/login">
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
