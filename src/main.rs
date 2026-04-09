//! Binary entrypoint for kjxlkj

use clap::{Parser, Subcommand};
use kjxlkj::{cli, config::Config, web};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(name = "kjxlkj")]
#[command(about = "Deterministic record service for LLM-operated workflows")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Documentation validation commands
    Docs {
        #[command(subcommand)]
        action: DocsAction,
    },
    /// Quality gate commands
    Quality {
        #[command(subcommand)]
        action: QualityAction,
    },
    /// Compose verification commands
    Compose {
        #[command(subcommand)]
        action: ComposeAction,
    },
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
enum DocsAction {
    /// Validate documentation topology
    ValidateTopology,
    /// Validate relative markdown links
    ValidateLinks,
    /// Validate canonical terms usage
    ValidateTerms,
}

#[derive(Subcommand)]
enum QualityAction {
    /// Check file line limits
    CheckLines,
}

#[derive(Subcommand)]
enum ComposeAction {
    /// Run compose verification
    Verify,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Docs { action }) => match action {
            DocsAction::ValidateTopology => cli::docs::validate_topology()?,
            DocsAction::ValidateLinks => cli::docs::validate_links()?,
            DocsAction::ValidateTerms => cli::docs::validate_terms()?,
        },
        Some(Commands::Quality { action }) => match action {
            QualityAction::CheckLines => cli::quality::check_lines()?,
        },
        Some(Commands::Compose { action }) => match action {
            ComposeAction::Verify => cli::compose::verify()?,
        },
        None => {
            let config = Config::from_env()?;
            info!(
                "Starting server on {}:{}",
                config.bind_host, config.bind_port
            );
            web::run_server(config).await?;
        }
    }

    Ok(())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "kjxlkj=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
