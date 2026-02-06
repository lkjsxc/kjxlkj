pub async fn run(file: Option<String>, headless: bool, script: Option<String>) -> anyhow::Result<()> {
    if headless {
        run_headless(file, script).await
    } else {
        run_tui(file).await
    }
}

async fn run_headless(_file: Option<String>, _script: Option<String>) -> anyhow::Result<()> {
    todo!("headless mode")
}

async fn run_tui(_file: Option<String>) -> anyhow::Result<()> {
    todo!("TUI mode")
}
