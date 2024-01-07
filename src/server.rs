use axum::{extract::Path, routing::get, Router};
use std::{net::SocketAddr, vec};
use url;

use crate::{
    crawl::crawler_start,
    readability::fetch_read,
    storage::{get_latest, insert_tops, Top},
};

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

    info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn crawl_handle() -> &'static str {
    let sites = vec![
        "hackernews".to_string(),
        "hupu".to_string(),
        "douban".to_string(),
    ];

    match get_latest(sites, None.unwrap_or(3600)) {
        Ok(mut m) => {
            match m.len() {
                0 => {
                    info!("latest links is empty, start crawls");
                    let crawl_result = crawler_start().unwrap();

                    info!("crawls resule: {:?}", crawl_result);
                    let mut tops: Vec<Top> = vec![];

                    for (k, v) in crawl_result {
                        for (i, item) in v.iter().enumerate() {
                            let top = Top {
                                url: item.url.clone(),
                                site: k.clone(),
                                title: item.title.clone(),
                                index: (i + 1) as i64,

                                id: 0,
                                created_at: chrono::Local::now().to_string(),
                            };

                            tops.push(top.clone());
                            if m.contains_key(k.as_str()) {
                                m.get_mut(k.as_str()).unwrap().push(top.clone());
                            } else {
                                m.insert(k.clone(), vec![top.clone()]);
                            }
                        }
                    }

                    match insert_tops(tops) {
                        Ok(t) => t,
                        Err(e) => {
                            error!("insert_tops error: {}", e);
                            return to_sstr("ERROR SERVER".to_string());
                        }
                    }
                }
                _ => {}
            }
            return to_sstr(serde_json::to_string_pretty(&m).unwrap());
        }
        Err(e) => {
            error!("get_latest_top_links error: {}", e);
            return to_sstr("ERROR SERVER".to_string());
        }
    }
}

async fn fetch_handle(Path(link): Path<String>) -> &'static str {
    let url = url::Url::parse(&link).unwrap();
    let readable = fetch_read(url.to_string()).await.unwrap();
    let json = serde_json::to_string_pretty(&readable).unwrap();

    to_sstr(json)
}

fn to_sstr(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
