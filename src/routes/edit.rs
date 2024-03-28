use crate::storage::{Article, Storage};
use crate::templates::{EditTemplate, HtmlTemplate};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::error;

pub async fn edit_route(Path(name): Path<String>) -> (StatusCode, impl IntoResponse) {
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

    let article = storage
        .articles
        .into_iter()
        .find(|a| a.title == name)
        .unwrap_or(Article {
            title: name,
            body: String::new(),
        });

    (
        StatusCode::OK,
        HtmlTemplate(EditTemplate {
            title: article.title,
            body: article.body,
        })
        .into_response(),
    )
}
