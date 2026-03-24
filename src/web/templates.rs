//! HTML templates

use crate::core::Record;

const CSS: &str = include_str!("templates/style.css");

pub fn base(title: &str, content: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title} - kjxlkj</title>
    <style>{CSS}</style>
</head>
<body>
{content}
</body>
</html>"#
    )
}

pub fn setup_page(error: Option<&str>) -> String {
    let error_html = error
        .map(|e| format!(r#"<div class="error">{e}</div>"#))
        .unwrap_or_default();
    let content = format!(
        r#"<div class="auth-container">
    <div class="auth-card">
        <h1>kjxlkj Setup</h1>
        <p class="subtitle">Create your admin account</p>
        {error_html}
        <form method="POST" action="/setup">
            <div class="form-group">
                <label for="username">Username</label>
                <input type="text" id="username" name="username" required minlength="3">
            </div>
            <div class="form-group">
                <label for="password">Password</label>
                <input type="password" id="password" name="password" required minlength="8">
            </div>
            <div class="form-group">
                <label for="confirm_password">Confirm Password</label>
                <input type="password" id="confirm_password" name="confirm_password" required>
            </div>
            <button type="submit" class="btn btn-primary">Create Account</button>
        </form>
    </div>
</div>"#
    );
    base("Setup", &content)
}

pub fn login_page(error: Option<&str>) -> String {
    let error_html = error
        .map(|e| format!(r#"<div class="error">{e}</div>"#))
        .unwrap_or_default();
    let content = format!(
        r#"<div class="auth-container">
    <div class="auth-card">
        <h1>kjxlkj</h1>
        <p class="subtitle">Sign in to admin panel</p>
        {error_html}
        <form method="POST" action="/login">
            <div class="form-group">
                <label for="username">Username</label>
                <input type="text" id="username" name="username" required>
            </div>
            <div class="form-group">
                <label for="password">Password</label>
                <input type="password" id="password" name="password" required>
            </div>
            <button type="submit" class="btn btn-primary">Sign In</button>
        </form>
    </div>
</div>"#
    );
    base("Login", &content)
}

pub fn home_page() -> String {
    let content = r#"<div class="auth-container">
    <div class="auth-card">
        <h1>kjxlkj</h1>
        <p class="subtitle">Deterministic Record Service</p>
        <p>Records available at <code>/v1/records</code></p>
        <a href="/login" class="btn btn-primary">Admin Login</a>
    </div>
</div>"#;
    base("Home", content)
}

pub fn admin_page(records: &[Record]) -> String {
    let rows: String = records.iter().map(|r| {
        let tags = r.tags.join(", ");
        format!(
            r#"<tr>
                <td><code>{}</code></td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td class="actions">
                    <button class="btn btn-sm" onclick="editRecord('{}')">Edit</button>
                    <button class="btn btn-sm btn-danger" onclick="deleteRecord('{}')">Delete</button>
                </td>
            </tr>"#,
            r.id, r.title, tags, r.revision, r.updated_at.format("%Y-%m-%d %H:%M"), r.id, r.id
        )
    }).collect();

    let empty_state = if records.is_empty() {
        r#"<tr><td colspan="6" class="empty">No records found</td></tr>"#
    } else {
        ""
    };

    let content = format!(
        r#"<div class="admin-layout">
    <nav class="sidebar">
        <div class="logo">kjxlkj admin</div>
        <ul>
            <li class="active"><a href="/admin">Records</a></li>
        </ul>
        <form method="POST" action="/logout" class="logout-form">
            <button type="submit" class="btn btn-sm">Logout</button>
        </form>
    </nav>
    <main class="main-content">
        <header class="page-header">
            <h1>Records</h1>
            <button class="btn btn-primary" onclick="createRecord()">Create Record</button>
        </header>
        <table class="records-table">
            <thead>
                <tr>
                    <th>ID</th>
                    <th>Title</th>
                    <th>Tags</th>
                    <th>Rev</th>
                    <th>Updated</th>
                    <th>Actions</th>
                </tr>
            </thead>
            <tbody>
                {rows}
                {empty_state}
            </tbody>
        </table>
    </main>
</div>
<div id="modal" class="modal hidden">
    <div class="modal-content">
        <h2 id="modal-title">Create Record</h2>
        <form id="record-form">
            <div class="form-group">
                <label for="record-id">ID</label>
                <input type="text" id="record-id" required pattern="[a-z0-9]+(?:-[a-z0-9]+)*" minlength="3" maxlength="48">
            </div>
            <div class="form-group">
                <label for="record-title">Title</label>
                <input type="text" id="record-title" required>
            </div>
            <div class="form-group">
                <label for="record-body">Body</label>
                <textarea id="record-body" rows="5"></textarea>
            </div>
            <div class="form-group">
                <label for="record-tags">Tags (comma-separated)</label>
                <input type="text" id="record-tags">
            </div>
            <div class="form-actions">
                <button type="button" class="btn" onclick="closeModal()">Cancel</button>
                <button type="submit" class="btn btn-primary">Save</button>
            </div>
        </form>
    </div>
</div>
<script>{script}</script>"#,
        script = include_str!("templates/admin.js")
    );
    base("Admin", &content)
}
