#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod routes;
mod storage;
pub mod templates;

use crate::routes::{article_route, delete_route, edit_route, root_route, update_route};
use axum::http::HeaderMap;
use axum::{routing::get, Router};
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::{warn, Level};
use tracing_subscriber::filter::LevelFilter;

pub const INSTANCE_NAME: &str = "NanoWIKI";

// Leave empty for no password
pub const EDIT_PASSWORD: &str = "CHANGEME";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .init();
    if EDIT_PASSWORD == "CHANGEME" {
        warn!("You are using the default password! Please change it");
    }
    if EDIT_PASSWORD.is_empty() {
        warn!("You did not set a password, this allows anyone to erase all pages, spam new pages etc.");
    }
    let app = Router::new()
        .route("/", get(root_route))
        .route("/style.css", get(style_route))
        .route("/favicon.ico", get(favicon_route))
        .route(
            "/articles/:name",
            get(article_route).post(update_route).delete(delete_route),
        )
        .route("/articles/:name/edit", get(edit_route))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn style_route() -> (HeaderMap, String) {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "text/css".parse().unwrap());
    (headers, include_str!("../static/style.css").to_string())
}

async fn favicon_route() -> &'static [u8] {
    include_bytes!("../static/favicon.ico")
}
