use crate::storage::Storage;
use crate::templates::{HtmlTemplate, IndexTemplate};
use axum::body::Body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;

pub async fn root_route() -> (StatusCode, Response<Body>) {
    let storage = match Storage::read().await {
        Ok(storage) => storage,
        Err(err) => {
            error!("Failed to read storage: {err}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to read storage: {err}").into_response(),
            );
        }
    };
    let mut articles = storage.articles;
    articles.sort_by_key(|a| a.title.clone());
    let article_names = articles
        .into_iter()
        .map(|a| html_escape::encode_text(&a.title).to_string())
        .collect();
    (
        StatusCode::OK,
        HtmlTemplate(IndexTemplate { article_names }).into_response(),
    )
}
