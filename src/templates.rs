use crate::storage::Article;
use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub article_names: Vec<String>,
}

#[derive(Template)]
#[template(path = "article.html", escape = "none")]
pub struct ArticleTemplate {
    pub title: String,
    pub body: String,
    pub creation_datetime: String,
    pub last_edit_datetime: String,
    pub edits: usize,
}

#[derive(Template)]
#[template(path = "edit.html")]
pub struct EditTemplate {
    pub article: Article,
    pub is_creation: bool,
}

pub struct HtmlTemplate<T>(pub T);

impl<T: Template> IntoResponse for HtmlTemplate<T> {
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template: {err}"),
            )
                .into_response(),
        }
    }
}
