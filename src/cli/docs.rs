use std::path::Path;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum DocsCmd {
    ValidateTopology,
}

pub fn run(command: DocsCmd) -> anyhow::Result<()> {
    match command {
        DocsCmd::ValidateTopology => {
            validate_readmes(Path::new("docs"))?;
            validate_readmes(Path::new("content"))?;
            println!("{{\"ok\":true,\"check\":\"docs_topology\"}}");
        }
    }
    Ok(())
}

fn validate_readmes(root: &Path) -> anyhow::Result<()> {
    for entry in walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
    {
        let path = entry.path();
        let mut kids = std::fs::read_dir(path)?
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        kids.retain(|k| !k.file_name().to_string_lossy().starts_with('.'));
        if kids.len() > 1 {
            let readmes = kids.iter().filter(|k| k.file_name() == "README.md").count();
            if readmes != 1 {
                anyhow::bail!("missing_or_multiple_readme: {}", path.display());
            }
        }
    }
    Ok(())
}
