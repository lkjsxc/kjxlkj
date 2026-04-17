//! Resource page template

use super::card_frame::status_pill;
use super::layout::{base, shell_page};
use super::model::{ResourceAnalytics, ResourceChrome};
use super::resource_editor::{editor_script, editor_surface};
use super::resource_focus::{analytics_block, live_resource_nav_strip};
use super::resource_media::{admin_media_panel, current_media_block};
use super::resource_shell::live_resource_rail;
use crate::web::db::{Resource, ResourceKind};
use crate::web::site::SiteContext;
use crate::web::view_media;

pub fn resource_page(
    resource: &Resource,
    chrome: &ResourceChrome,
    analytics: Option<&ResourceAnalytics>,
    body_html: &str,
    is_admin: bool,
    site: &SiteContext,
) -> String {
    let content = format!(
        r#"<header class="page-head resource-head">
<div class="page-meta">
<small><span>Created</span>{}</small>
<small><span>Updated</span>{}</small>
{}
</div>
</header>
{}{}{}"#,
        chrome.created_at,
        chrome.updated_at,
        status_pill(chrome.visibility, ""),
        live_resource_nav_strip(chrome, is_admin),
        analytics_block(analytics),
        resource_body(resource, chrome, body_html, is_admin),
    );
    let page_meta = site
        .page_meta(
            &chrome.title,
            resource.summary.clone(),
            !is_admin && !resource.is_private,
            (!is_admin && !resource.is_private).then_some(chrome.current_href.as_str()),
        )
        .with_social_card(
            (!is_admin && !resource.is_private)
                .then(|| {
                    view_media::social_card_href(resource).and_then(|href| site.absolute_url(&href))
                })
                .flatten(),
        );
    base(
        &page_meta,
        &shell_page(
            if is_admin { "Admin" } else { "Guest" },
            &live_resource_rail(chrome, is_admin),
            &content,
            "resource-page",
            &site.site_name,
        ),
        "",
        &editor_script(resource, chrome, is_admin, &site.site_name),
    )
}

fn resource_body(resource: &Resource, chrome: &ResourceChrome, body_html: &str, is_admin: bool) -> String {
    if is_admin {
        return format!(
            "{}{}",
            if resource.kind == ResourceKind::Media {
                admin_media_panel(resource)
            } else {
                String::new()
            },
            editor_surface(resource, chrome),
        );
    }
    format!(
        r#"{}<section class="surface resource-surface prose">{}</section>"#,
        if resource.kind == ResourceKind::Media {
            current_media_block(resource)
        } else {
            String::new()
        },
        body_html
    )
}
