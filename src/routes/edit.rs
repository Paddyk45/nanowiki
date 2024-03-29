use crate::storage::{Article, Storage};
use crate::templates::{EditTemplate, HtmlTemplate};
use axum::body::Body;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;

pub async fn edit_route(Path(name): Path<String>) -> (StatusCode, Response<Body>) {
    let name = name.trim().replace(['/', '"'], "");
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
            title: name.to_string(),
            ..Default::default()
        });

    let is_creation = article.creation_timestamp == 0;
    (
        StatusCode::OK,
        HtmlTemplate(EditTemplate {
            article,
            is_creation,
        })
        .into_response(),
    )
}
