pub(super) fn settings_row(label: &str, body: &str, class_name: &str) -> String {
    format!(
        r#"<section class="settings-row-block {class_name}" data-settings-row>
<div class="settings-row-title">{label}</div>
<div class="settings-row-content">{body}</div>
</section>"#
    )
}
