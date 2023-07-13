use axum::{extract::Path, routing::get, Router};
use readability::extractor;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use url;

use crate::crawl;

pub async fn start(port: u16) {
    let cors_middleware = tower_http::cors::CorsLayer::new()
        .allow_origin(
            "http://localhost:3000"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([axum::http::Method::GET]);

    let app = Router::new()
        .route("/crawl", get(crawl_handle))
        .route("/fetch/:url", get(fetch_handle))
        .layer(cors_middleware);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    log::info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

const CRAWL_CACHE: &'static str = include_str!("/Users/abcdlsj/Workspace/mofish/mofish.json");

async fn crawl_handle() -> &'static str {
    if CRAWL_CACHE.len() > 0 {
        return CRAWL_CACHE;
    }

    let map = crawl::crawler();
    let json = serde_json::to_string_pretty(&map).unwrap();

    string_to_static_str(json)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadableLink {
    url: String,
    title: String,
    content: String,
}

async fn fetch_handle(Path(link): Path<String>) -> &'static str {
    let url = url::Url::parse(&link).unwrap();
    let readable = fetch_read(url.to_string()).await;
    let json = serde_json::to_string_pretty(&readable).unwrap();

    string_to_static_str(json)
}

async fn fetch_read(url: String) -> ReadableLink {
    let ret = extractor::scrape(&url).unwrap();

    ReadableLink {
        url: url,
        title: ret.title,
        content: ret.content,
    }
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
