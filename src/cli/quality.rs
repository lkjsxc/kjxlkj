use clap::Subcommand;

#[derive(Subcommand)]
pub enum QualityCmd {
    CheckLines,
}

pub fn run(command: QualityCmd) -> anyhow::Result<()> {
    match command {
        QualityCmd::CheckLines => {
            check_max_lines(".", 300, &["md"])?;
            check_max_lines("src", 200, &["rs"])?;
            check_max_lines("templates", 200, &["html"])?;
            check_max_lines("static", 200, &["js", "css"])?;
            check_max_lines("migrations", 200, &["sql"])?;
            println!("{{\"ok\":true,\"check\":\"line_limits\"}}");
        }
    }
    Ok(())
}

fn check_max_lines(root: &str, max: usize, exts: &[&str]) -> anyhow::Result<()> {
    for entry in walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if path.iter().any(|s| s == ".git" || s == "target") {
            continue;
        }
        let ext = path
            .extension()
            .and_then(|x| x.to_str())
            .unwrap_or_default();
        if !exts.contains(&ext) {
            continue;
        }
        let lines = std::fs::read_to_string(path)?.lines().count();
        if lines > max {
            anyhow::bail!(
                "line_limit_exceeded: {} has {} > {}",
                path.display(),
                lines,
                max
            );
        }
    }
    Ok(())
}
