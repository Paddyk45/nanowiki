use crate::storage::Storage;
use crate::EDIT_PASSWORD;
use axum::extract::Path;
use axum::http::StatusCode;
use tracing::error;

pub async fn delete_route(
    Path(name): Path<String>,
    body: String,
) -> (StatusCode, String) {
    if !EDIT_PASSWORD.is_empty() && body != format!("pw={EDIT_PASSWORD}") {
        return (
            StatusCode::FORBIDDEN,
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

    storage.articles.retain(|a| a.title != name);

    if let Err(err) = storage.write().await {
        error!("Failed to write storage: {err}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to write storage: {err}"),
        );
    }

    (StatusCode::OK, "Article was deleted".to_string())
}
