use clap::Subcommand;
use serde_json::json;

use crate::adapters::content_store::FsContentStore;

#[derive(Subcommand)]
pub enum ContentCmd {
    List {
        #[arg(long)]
        include_private: bool,
    },
    Validate,
    SetPrivate {
        slug: String,
        #[arg(long)]
        value: bool,
    },
}

pub async fn run(command: ContentCmd) -> anyhow::Result<()> {
    let store = FsContentStore::new("content/articles");
    match command {
        ContentCmd::List { include_private } => {
            let out = store.list(include_private).await?;
            println!("{}", serde_json::to_string_pretty(&out)?);
        }
        ContentCmd::Validate => {
            let out = store.list(true).await?;
            println!("{}", json!({"ok": true, "articles": out.len()}));
        }
        ContentCmd::SetPrivate { slug, value } => {
            let article = store
                .get(&slug)
                .await?
                .ok_or_else(|| anyhow::anyhow!("article_not_found"))?;
            store
                .save(&slug, &article.title, &article.body, value)
                .await?;
            println!("{}", json!({"ok": true, "slug": slug, "private": value}));
        }
    }
    Ok(())
}
