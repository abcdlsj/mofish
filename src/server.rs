use axum::{routing::get, Router};
use std::net::SocketAddr;

use crate::crawl;

pub async fn start(port: u16) {
    let app = Router::new().route("/crawl", get(crawl_handle)).layer(
        tower_http::cors::CorsLayer::new()
            .allow_origin(
                "http://localhost:3000"
                    .parse::<axum::http::HeaderValue>()
                    .unwrap(),
            )
            .allow_methods([axum::http::Method::GET]),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    log::info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn crawl_handle() -> &'static str {
    let map = crawl::crawler();
    let json = serde_json::to_string_pretty(&map).unwrap();

    string_to_static_str(json)
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
