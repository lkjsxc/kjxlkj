use crate::core::content::draft_title_and_slug;
use crate::web::handlers::app_shell::render_shell_page;
use crate::web::handlers::page_html::escape_html;

pub fn render_admin_shell(
    site_title: &str,
    slugs: &[String],
    now: chrono::DateTime<chrono::Utc>,
) -> String {
    let (draft_title, draft_slug) = draft_title_and_slug(now);
    let list_items = slugs
        .iter()
        .map(|slug| {
            let escaped = escape_html(slug);
            format!(
                r##"<li><a href="/article/{escaped}" data-slug="{escaped}">{escaped}</a></li>"##
            )
        })
        .collect::<String>();

    let main = format!(
        r#"<main id="admin-page">
    <h1>Admin articles</h1>
    <aside>
      <h2>Articles</h2>
      <input id="admin-quick-open" type="search" placeholder="Filter articles" />
      <section id="admin-create-panel">
        <form id="admin-create-form" method="post" action="/admin/create">
          <label for="admin-create-slug">New slug</label>
          <input id="admin-create-slug" name="slug" type="text" value="{draft_slug}" />
          <label for="admin-create-title">Title</label>
          <input id="admin-create-title" name="title" type="text" value="{draft_title}" />
          <label for="admin-create-private">Private</label>
          <input id="admin-create-private" name="private" type="checkbox" value="true" checked />
          <input name="body" type="hidden" value="" />
          <button type="submit">Create</button>
        </form>
      </section>
      <section id="admin-article-list">
        <ol>{list_items}</ol>
      </section>
    </aside>
  </main>"#
    );
    render_shell_page(site_title, "Admin", &main, slugs, true)
}
