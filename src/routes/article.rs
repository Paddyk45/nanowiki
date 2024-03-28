use crate::storage::Storage;
use crate::templates::{ArticleTemplate, HtmlTemplate};
use axum::body::Body;
use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use tracing::error;
use pulldown_cmark::Parser;

pub async fn article_route(Path(name): Path<String>) -> (StatusCode, HeaderMap, Response<Body>) {
    let storage = match Storage::read().await {
        Ok(storage) => storage,
        Err(err) => {
            error!("Failed to read storage: {err}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                HeaderMap::new(),
                format!("Failed to read storage: {err}").into_response(),
            );
        }
    };
    let Some(article) = storage.articles.iter().find(|a| a.title.eq(&name)) else {
        return storage
            .articles
            .iter()
            .find(|a| a.title.eq_ignore_ascii_case(&name))
            .map_or_else(
                || {
                    (
                        StatusCode::NOT_FOUND,
                        HeaderMap::new(),
                        "Unknown article".into_response(),
                    )
                },
                |correct_article| {
                    let mut headers = HeaderMap::new();
                    headers.insert(
                        "location",
                        format!("/articles/{}", correct_article.title)
                            .parse()
                            .unwrap(),
                    );
                    (
                        StatusCode::MOVED_PERMANENTLY,
                        headers,
                        String::new().into_response(),
                    )
                },
            );
    };

    let parser = Parser::new(&article.body);
    let mut cmark_body = String::new();
    pulldown_cmark::html::push_html(&mut cmark_body, parser);

    let template = ArticleTemplate {
        title: article.title.clone(),
        body: cmark_body,
    };

    (
        StatusCode::OK,
        HeaderMap::new(),
        HtmlTemplate(template).into_response(),
    )
}
