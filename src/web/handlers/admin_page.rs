use crate::web::handlers::admin_fragments::{
    escape_html, render_admin_editor_pane, render_admin_preview_empty,
};
use crate::web::handlers::app_shell::render_shell_page;

pub fn render_admin_shell(site_title: &str, slugs: &[String]) -> String {
    let list_items = slugs
        .iter()
        .map(|slug| {
            let escaped = escape_html(slug);
            format!(
                r##"<li><a href="/admin/open/{escaped}" data-admin-open="true" data-slug="{escaped}" hx-get="/admin/open/{escaped}" hx-target="#admin-editor-pane" hx-swap="outerHTML">{escaped}</a></li>"##
            )
        })
        .collect::<String>();
    let editor = render_admin_editor_pane("", None, "", false, "", false);
    let preview = render_admin_preview_empty(false);

    let main = format!(
        r#"<main id="admin-page">
    <h1>Admin editor</h1>
    <section id="admin-status-banner" aria-live="polite"></section>
    <section id="admin-conflict-banner" role="alert" aria-live="assertive" data-conflict="false"></section>
    <section id="admin-unsaved-indicator" aria-live="polite" data-unsaved="false">All changes saved</section>
    <aside>
      <h2>Articles</h2>
      <input id="admin-quick-open" type="search" placeholder="Filter articles" />
      <section id="admin-create-panel" hidden>
        <form id="admin-create-form" method="post" action="/admin/create" data-admin-nav-form="create">
          <label for="admin-create-slug">New slug</label>
          <input id="admin-create-slug" name="slug" type="text" />
          <input name="title" type="hidden" value="" />
          <input name="body" type="hidden" value="" />
          <button type="submit">Create</button>
        </form>
      </section>
      <section id="admin-article-list">
        <ol>{list_items}</ol>
      </section>
    </aside>
    {editor}
    {preview}
  </main><script src="/static/admin-runtime-core.js" defer></script><script src="/static/admin-runtime-autosave.js" defer></script><script src="/static/admin-runtime-shortcuts.js" defer></script>"#
    );
    render_shell_page(site_title, "Admin editor", &main, slugs, true)
}
