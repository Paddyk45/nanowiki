use crate::storage::{Article, Storage};
use crate::EDIT_PASSWORD;
use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::Form;
use serde::Deserialize;
use tracing::error;

#[derive(Deserialize)]
pub struct FormParams {
    pw: Option<String>,
    content: String,
}

pub async fn update_route(
    Path(name): Path<String>,
    Form(form): Form<FormParams>,
) -> (StatusCode, String) {
    let name = name.trim();
    if name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            "Article name cannot be empty".to_string(),
        );
    }

    if !EDIT_PASSWORD.is_empty() && !form.pw.is_some_and(|pw| pw == EDIT_PASSWORD) {
        return (
            StatusCode::BAD_REQUEST,
            "Wrong or missing password".to_string(),
        );
    }

    let mut storage = match Storage::read().await {
        Ok(storage) => storage,
        Err(err) => {
            error!("Failed to read storage: {err}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to read storage: {err}"),
            );
        }
    };

    if let Some(article) = storage.articles.iter_mut().find(|a| a.title == name) {
        article.body = form.content;
    } else {
        storage.articles.push(Article {
            title: name.to_string(),
            body: form.content.to_string(),
        });
    };

    if let Err(err) = storage.write().await {
        error!("Failed to write storage: {err}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to write storage: {err}"),
        );
    }

    let mut headers = HeaderMap::new();
    headers.insert("location", format!("/articles/{name}").parse().unwrap());
    (
        StatusCode::OK,
        "Article was updated successfully!".to_string(),
    )
}
