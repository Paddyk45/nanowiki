use crate::storage::Storage;
use crate::templates::{ArticleTemplate, HtmlTemplate};
use axum::body::Body;
use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use pulldown_cmark::Parser;
use tracing::error;

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
    let Some(article) = storage
        .articles
        .iter()
        .find(|a| a.title.eq_ignore_ascii_case(&name))
    else {
        if let Some(correct_article) = storage.articles.iter().find(|a| {
            a.title
                .eq_ignore_ascii_case(&html_escape::decode_html_entities(&name))
        }) {
            let mut headers = HeaderMap::new();
            headers.insert(
                "location",
                format!("/articles/{}", correct_article.title)
                    .parse()
                    .unwrap(),
            );
            return (
                StatusCode::MOVED_PERMANENTLY,
                headers,
                String::new().into_response(),
            );
        }
        return (
            StatusCode::NOT_FOUND,
            HeaderMap::new(),
            "Unknown article".into_response(),
        );
    };
    let encoded_body = html_escape::encode_text(&article.body).to_string();
    let parser = Parser::new(&encoded_body);
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
