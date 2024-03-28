use crate::storage::{Article, Storage};
use crate::EDIT_PASSWORD;
use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use tracing::error;

pub async fn update_route(
    Path(name): Path<String>,
    body: String,
) -> (StatusCode, HeaderMap, String) {
    let name = name.trim();
    if name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            HeaderMap::new(),
            "Article name cannot be empty".to_string(),
        );
    }
    let body = urlencoding::decode(&body)
        .unwrap_or_default()
        .replace('+', " ");
    let Some(body) = body.strip_prefix("content=") else {
        return (
            StatusCode::BAD_REQUEST,
            HeaderMap::new(),
            "Body does not start with content=".to_string(),
        );
    };
    let content = if EDIT_PASSWORD.is_empty() {
        body
    } else {
        let mut split = body.split("&password=");
        let content: &str = split.next().unwrap_or_default();
        let password: &str = split.next().unwrap_or_default();
        if password != EDIT_PASSWORD {
            return (
                StatusCode::UNAUTHORIZED,
                HeaderMap::new(),
                "Wrong password".to_string(),
            );
        }
        content
    };

    let mut storage = match Storage::read().await {
        Ok(storage) => storage,
        Err(err) => {
            error!("Failed to read storage: {err}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                HeaderMap::new(),
                format!("Failed to read storage: {err}"),
            );
        }
    };

    match storage.articles.iter_mut().find(|a| a.title == name) {
        Some(article) => {
            article.body = html_escape::encode_text(content).to_string();
        }
        None => {
            storage.articles.push(Article {
                title: name.to_string(),
                body: html_escape::encode_text(content).to_string(),
            });
        }
    };

    if let Err(err) = storage.write().await {
        error!("Failed to write storage: {err}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            HeaderMap::new(),
            format!("Failed to write storage: {err}"),
        );
    }
    let mut headers = HeaderMap::new();
    headers.insert("location", format!("/articles/{name}").parse().unwrap());
    (
        StatusCode::SEE_OTHER,
        headers,
        "Article was updated successfully!".to_string(),
    )
}
