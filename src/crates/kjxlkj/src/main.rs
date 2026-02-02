//! kjxlkj - A modal text editor.

#![allow(dead_code)]

mod app;

use clap::Parser;

/// Command line arguments for kjxlkj.
#[derive(Parser, Debug)]
#[command(name = "kjxlkj")]
#[command(version, about = "A modal text editor", long_about = None)]
struct Args {
    /// Files to open.
    #[arg(value_name = "FILE")]
    files: Vec<String>,

    /// Start in read-only mode.
    #[arg(short = 'R', long)]
    readonly: bool,

    /// Execute command after loading.
    #[arg(short = 'c', long = "cmd")]
    command: Option<String>,

    /// Go to line number.
    #[arg(short = '+')]
    line: Option<usize>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Run the application
    let mut app = if let Some(file) = args.files.first() {
        app::Application::with_file(std::path::Path::new(file))?
    } else {
        app::Application::new()?
    };
    app.run()?;
    
    Ok(())
}
