mod markup;
mod storage;
mod model;

use askama::Template;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use axum::response::{Html, IntoResponse, Response};
use url::Url;
use crate::markup::MarkupElement;

const INSTANCE_NAME: &'static str = "NanoWIKI";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> impl IntoResponse {
}

#[derive(Template)]
#[template(path = "article.html", escape = "none")]
struct ArticleTemplate {
    instance_name: String,
    title: String,
    body: String,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
    where
        T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}