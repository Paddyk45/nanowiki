use crate::storage::{Article, Storage};
use crate::EDIT_PASSWORD;
use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::Form;
use chrono::Utc;
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
    if !EDIT_PASSWORD.is_empty() && !form.pw.is_some_and(|pw| pw == EDIT_PASSWORD) {
        return (
            StatusCode::UNAUTHORIZED,
            "Wrong or missing password".to_string(),
        );
    }

    let name = name.trim().replace(['/', '"'], "");
    let name = html_escape::decode_html_entities(&name);
    if name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            "Article name cannot be empty".to_string(),
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
        article.last_edit_timestamp = Some(Utc::now().timestamp());
        article.edits += 1;
    } else {
        storage.articles.push(Article {
            title: name.to_string(),
            body: form.content.to_string(),
            creation_timestamp: Utc::now().timestamp(),
            last_edit_timestamp: None,
            edits: 0,
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
