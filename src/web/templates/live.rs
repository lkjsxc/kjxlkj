//! Live broadcast template

use super::index::{admin_create_actions, list_rail};
use super::layout::{base, html_escape, shell_page};
use super::sections::page_header;
use crate::web::site::SiteContext;

const LIVE_JS: &str = include_str!("live.js");

pub fn live_page(
    site: &SiteContext,
    is_admin: bool,
    login_href: &str,
    ice_servers: &serde_json::Value,
) -> String {
    let actions = if is_admin {
        r#"<form method="POST" action="/logout"><button type="submit" class="btn">Logout</button></form>"#.to_string()
    } else {
        format!(
            r#"<a href="{}" class="btn">Admin sign in</a>"#,
            html_escape(login_href)
        )
    };
    let admin_actions = if is_admin {
        admin_create_actions()
    } else {
        String::new()
    };
    let content = format!(
        "{}{}{}",
        page_header("Live", None, "live-head"),
        live_surface(is_admin),
        ice_config(ice_servers),
    );
    base(
        &site.page_meta("Live", "Public live broadcast.", false, None),
        &shell_page(
            if is_admin { "Admin" } else { "Public" },
            &list_rail("live", &admin_actions, &actions, is_admin),
            &content,
            "live-page",
            &site.site_name,
        ),
        "",
        &format!(r#"<script>{LIVE_JS}</script>"#),
    )
}

fn live_surface(is_admin: bool) -> String {
    let admin_controls = if is_admin {
        r#"<div class="live-controls">
<button type="button" class="btn btn-primary" data-live-start>Start broadcast</button>
<button type="button" class="btn" data-live-stop disabled>Stop broadcast</button>
</div>"#
    } else {
        ""
    };
    format!(
        r#"<section class="surface live-surface" data-live-root data-live-role="{}">
<div class="live-video-wrap">
<video class="live-video" autoplay playsinline muted data-live-video></video>
</div>
<div class="live-status-row">
<strong data-live-state>Waiting for broadcast</strong>
<span data-live-detail>No active stream.</span>
</div>
{admin_controls}
</section>"#,
        if is_admin { "broadcaster" } else { "viewer" },
    )
}

fn ice_config(ice_servers: &serde_json::Value) -> String {
    format!(
        r#"<script type="application/json" id="live-ice-servers">{}</script>"#,
        html_escape(&ice_servers.to_string()),
    )
}
