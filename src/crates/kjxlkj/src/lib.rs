//! kjxlkj editor — main application wiring.

mod headless;
mod tui;

pub async fn run(
    file: Option<String>,
    headless: bool,
    script: Option<String>,
) -> anyhow::Result<()> {
    if headless {
        headless::run_headless(file, script).await
    } else {
        tui::run_tui(file).await
    }
}
