use crate::storage::Storage;
use crate::templates::{HtmlTemplate, IndexTemplate};
use axum::body::Body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;
use crate::{MODE, Mode};

pub async fn index_route() -> (StatusCode, Response<Body>) {
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
    articles.sort_by_key(|a| {
        if MODE == Mode::Blog {
            (i64::MAX - a.creation_timestamp).to_string()
        } else {
            a.title.clone()
        }
    });
    let articles = articles
        .into_iter()
        .map(|a| (a.title.clone(), a.creation_time_rel()))
        .collect();
    (
        StatusCode::OK,
        HtmlTemplate(IndexTemplate { articles }).into_response(),
    )
}
