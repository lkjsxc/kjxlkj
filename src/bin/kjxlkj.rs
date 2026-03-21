use clap::{Parser, Subcommand};

use kjxlkj::{app, cli};

#[derive(Parser)]
#[command(name = "kjxlkj")]
struct Cmd {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Serve,
    Content {
        #[command(subcommand)]
        command: cli::content::ContentCmd,
    },
    Docs {
        #[command(subcommand)]
        command: cli::docs::DocsCmd,
    },
    Quality {
        #[command(subcommand)]
        command: cli::quality::QualityCmd,
    },
    System {
        #[command(subcommand)]
        command: cli::system::SystemCmd,
    },
    Compose {
        #[command(subcommand)]
        command: cli::compose::ComposeCmd,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let cmd = Cmd::parse();

    match cmd.command {
        Command::Serve => {
            let bind_addr =
                std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
            let db_url = std::env::var("DATABASE_URL")?;
            app::run(&bind_addr, &db_url).await
        }
        Command::Content { command } => cli::content::run(command).await,
        Command::Docs { command } => cli::docs::run(command),
        Command::Quality { command } => cli::quality::run(command),
        Command::System { command } => cli::system::run(command).await,
        Command::Compose { command } => cli::compose::run(command),
    }
}
