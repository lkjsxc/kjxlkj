//! Live broadcast template

use super::index::{admin_create_actions, list_rail};
use super::layout::{base, html_escape, shell_page};
use super::sections::page_header;
use crate::web::db::AppSettings;
use crate::web::site::SiteContext;
use serde_json::json;

const LIVE_STATE_JS: &str = include_str!("live_state.js");
const LIVE_DEVICES_JS: &str = include_str!("live_devices.js");
const LIVE_CAPTURE_JS: &str = include_str!("live_capture.js");
const LIVE_PEER_JS: &str = include_str!("live_peer.js");
const LIVE_JS: &str = include_str!("live.js");

pub fn live_page(
    site: &SiteContext,
    is_admin: bool,
    login_href: &str,
    settings: &AppSettings,
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
        live_surface(is_admin, settings),
        live_config(settings),
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
        &format!(
            r#"<script>{LIVE_STATE_JS}</script><script>{LIVE_DEVICES_JS}</script><script>{LIVE_CAPTURE_JS}</script><script>{LIVE_PEER_JS}</script><script>{LIVE_JS}</script>"#
        ),
    )
}

fn live_surface(is_admin: bool, settings: &AppSettings) -> String {
    let admin_controls = if is_admin {
        format!(
            r#"<div class="live-controls">
<div class="live-control-grid">
<label class="form-group"><span>Source</span><select data-live-source>{}</select></label>
<label class="form-group"><span>Camera facing</span><select data-live-camera-facing>{}</select></label>
<label class="form-group"><span>Camera</span><select data-live-camera></select></label>
<label class="form-group"><span>Quality</span><select data-live-height>{}</select></label>
<label class="form-group"><span>Frame rate</span><select data-live-fps>{}</select></label>
<label class="check-row live-mic"><input type="checkbox" data-live-mic {}><span>Microphone</span></label>
</div>
<div class="live-action-row">
<button type="button" class="btn btn-primary" data-live-start>Start broadcast</button>
<button type="button" class="btn" data-live-stop disabled>Stop broadcast</button>
<span class="status-pill" data-live-viewer-count>0 viewers</span>
</div>
</div>"#,
            source_options(&settings.live_default_source),
            camera_facing_options(&settings.live_default_camera_facing),
            number_options(
                &[360, 480, 720, 1080, 1440, 2160],
                settings.live_default_height,
                "p"
            ),
            number_options(&[15, 30, 45, 60, 120], settings.live_default_fps, " fps"),
            if settings.live_default_microphone_enabled {
                "checked"
            } else {
                ""
            },
        )
    } else {
        String::new()
    };
    format!(
        r#"<section class="surface live-surface" data-live-root data-live-role="{}">
<div class="live-video-wrap">
<video class="live-video" autoplay playsinline controls{} data-live-video></video>
</div>
<div class="live-status-row">
<strong data-live-state>Waiting for broadcast</strong>
<span data-live-detail>No active stream.</span>
</div>
{admin_controls}
</section>"#,
        if is_admin { "broadcaster" } else { "viewer" },
        if is_admin { " muted" } else { "" },
    )
}

fn live_config(settings: &AppSettings) -> String {
    let config = json!({
        "source": settings.live_default_source,
        "cameraFacing": settings.live_default_camera_facing,
        "height": settings.live_default_height,
        "fps": settings.live_default_fps,
        "microphone": settings.live_default_microphone_enabled,
    });
    format!(
        r#"<script type="application/json" id="live-config">{}</script>"#,
        script_json(&config.to_string()),
    )
}

fn script_json(value: &str) -> String {
    value
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('&', "\\u0026")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029")
}

fn source_options(current: &str) -> String {
    [("screen", "Screen"), ("camera", "Camera")]
        .into_iter()
        .map(|(value, label)| option(value, label, value == current))
        .collect::<Vec<_>>()
        .join("")
}

fn camera_facing_options(current: &str) -> String {
    [("environment", "Rear"), ("user", "Front")]
        .into_iter()
        .map(|(value, label)| option(value, label, value == current))
        .collect::<Vec<_>>()
        .join("")
}

fn number_options(values: &[i64], current: i64, suffix: &str) -> String {
    values
        .iter()
        .map(|value| {
            option(
                &value.to_string(),
                &format!("{value}{suffix}"),
                *value == current,
            )
        })
        .collect::<Vec<_>>()
        .join("")
}

fn option(value: &str, label: &str, selected: bool) -> String {
    format!(
        r#"<option value="{}"{}>{}</option>"#,
        html_escape(value),
        if selected { " selected" } else { "" },
        html_escape(label),
    )
}
