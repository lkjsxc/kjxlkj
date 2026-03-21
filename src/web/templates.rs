use askama::Template;

use crate::adapters::content_store::Article;

#[derive(Template)]
#[template(path = "public/index.html")]
pub struct PublicIndexTemplate {
    pub articles: Vec<Article>,
}

#[derive(Template)]
#[template(path = "public/article.html")]
pub struct PublicArticleTemplate {
    pub title: String,
    pub body_html: String,
}

#[derive(Template)]
#[template(path = "auth/page.html")]
pub struct AuthTemplate<'a> {
    pub mode: &'a str,
    pub action: &'a str,
    pub message: &'a str,
}

#[derive(Template)]
#[template(path = "admin/home.html")]
pub struct AdminTemplate {
    pub articles: Vec<Article>,
}
