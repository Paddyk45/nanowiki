use crate::storage::Storage;
use crate::templates::{ArticleTemplate, HtmlTemplate};
use axum::body::Body;
use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use chrono::TimeZone;
use pulldown_cmark::Parser;
use tracing::error;

pub async fn article_route(Path(name): Path<String>) -> (StatusCode, HeaderMap, Response<Body>) {
    let name = name.trim().replace(['/', '"'], "");
    let storage = match Storage::read().await {
        Ok(storage) => storage,
        Err(err) => {
            error!("Failed to read storage: {err}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                HeaderMap::new(),
                format!("Failed to read storage: {err}").into_response(),
            );
        }
    };
    let Some(article) = storage
        .articles
        .iter()
        .find(|a| a.title.eq_ignore_ascii_case(&name))
    else {
        if let Some(correct_article) = storage.articles.iter().find(|a| {
            a.title
                .eq_ignore_ascii_case(&name)
        }) {
            let mut headers = HeaderMap::new();
            headers.insert(
                "location",
                format!("/articles/{}", correct_article.title)
                    .parse()
                    .unwrap(),
            );
            return (
                StatusCode::MOVED_PERMANENTLY,
                headers,
                String::new().into_response(),
            );
        }
        return (
            StatusCode::NOT_FOUND,
            HeaderMap::new(),
            "Unknown article".into_response(),
        );
    };
    
    let lines = html_escape::encode_text(&article.body)
        .lines()
        .map(String::from)
        .map(|mut l| {
            if let Some(s) = l.strip_prefix("&gt; ") {
                l = format!("> {s}")
            }
            l
        })
        .collect::<Vec<String>>()
        .join("\n");
    let parser = Parser::new(&lines);
    let mut cmark_body = String::new();
    pulldown_cmark::html::push_html(&mut cmark_body, parser);

    let creation_datetime = chrono::Utc
        .timestamp_opt(article.creation_timestamp, 0)
        .unwrap()
        .to_string();
    let last_edit_datetime = article
        .last_edit_timestamp
        .map(|ts| chrono::Utc.timestamp_opt(ts, 0).unwrap().to_string())
        .unwrap_or_default();

    let template = ArticleTemplate {
        title: article.title.clone(),
        body: cmark_body,
        creation_datetime,
        last_edit_datetime,
        edits: article.edits,
    };

    (
        StatusCode::OK,
        HeaderMap::new(),
        HtmlTemplate(template).into_response(),
    )
}
