use super::index::{admin_create_actions, list_rail};
use super::layout::{base, shell_page};
use super::sections::page_header;
use crate::web::site::SiteContext;

const MEDIA_NEW_JS: &str = include_str!("media_new.js");

pub fn media_new_page(default_private: bool, site: &SiteContext) -> String {
    let admin_actions = admin_create_actions();
    let content = format!(
        r#"{}<section class="surface settings-panel"><form id="media-create-form" class="settings-form settings-stack">
<label class="form-group"><span>File</span><input id="media-file-input" type="file" name="file" accept="image/*,video/*" required></label>
<label class="form-group"><span>Alias</span><input id="media-alias-input" type="text" name="alias" maxlength="64" placeholder="Optional route alias"></label>
<label class="check-row check-row-field"><input id="media-favorite-toggle" type="checkbox" name="is_favorite"><span>Favorite</span></label>
<label class="check-row check-row-field"><input id="media-public-toggle" type="checkbox" {}><span>Public</span></label>
<p id="media-create-error" class="error" hidden></p>
<div class="settings-submit-row"><button id="media-create-submit" type="submit" class="btn btn-primary">Create media</button><a href="/admin" class="btn">Back to dashboard</a></div>
</form></section>"#,
        page_header("New Media", None, "settings-head"),
        if default_private { "" } else { "checked" },
    );
    base(
        &site.page_meta(
            "New Media",
            format!("Upload a new media resource for {}.", site.site_name),
            false,
            None,
        ),
        &shell_page(
            "Admin",
            &list_rail(
                "admin",
                &admin_actions,
                r#"<form method="POST" action="/logout"><button type="submit" class="btn">Logout</button></form>"#,
                true,
            ),
            &content,
            "settings-page",
            &site.site_name,
        ),
        "",
        &format!(r#"<script>{MEDIA_NEW_JS}</script>"#),
    )
}
