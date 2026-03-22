use crate::web::handlers::page_html::escape_html;
use crate::web::state::ArticleHistory;

use super::time_format::format_utc_timestamp;

pub fn render_history_main(history: &ArticleHistory, restore_enabled: bool) -> String {
    let rows = history
        .entries
        .iter()
        .map(|entry| {
            let restore = if restore_enabled {
                format!(
                    "<form method=\"post\" action=\"/article/{}/history/restore\"><input type=\"hidden\" name=\"commit_id\" value=\"{}\" /><button type=\"submit\">Restore</button></form>",
                    escape_html(&history.slug),
                    escape_html(&entry.commit_id)
                )
            } else {
                String::new()
            };
            format!(
                "<li><code>{}</code><time>{}</time><p>{}</p>{}</li>",
                escape_html(&entry.commit_id),
                escape_html(&format_utc_timestamp(entry.committed_at)),
                escape_html(&entry.message),
                restore
            )
        })
        .collect::<String>();
    format!(
        "<main id=\"article-history-page\"><h1>History: {}</h1><section id=\"article-history-list\"><ol>{}</ol></section></main>",
        escape_html(&history.slug),
        rows
    )
}
