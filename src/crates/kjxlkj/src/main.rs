use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "kjxlkj", version, about = "Neovim-inspired TUI text editor")]
struct Cli {
    /// File to open
    file: Option<String>,
    /// Run in headless mode (no TUI)
    #[arg(long)]
    headless: bool,
    /// Run a script file in headless mode
    #[arg(long)]
    script: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        kjxlkj::run(cli.file, cli.headless, cli.script).await
    })
}
