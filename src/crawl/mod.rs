use anyhow::{Ok, Result};
use reqwest;
use scraper;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;

pub mod config;
pub mod douban;
pub mod hn;
pub mod hupu;

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub title: String,
    pub url: String,
}

pub trait Crawler {
    fn crawl(&self, url: &str) -> Vec<Item> {
        let document = self.request(url);

        self.parse(&document)
    }

    fn selector(&self) -> scraper::Selector;

    fn request(&self, url: &str) -> scraper::Html {
        let body = tokio::task::block_in_place(|| {
            let body = reqwest::blocking::get(url).unwrap().text().unwrap();

            body
        });

        return scraper::Html::parse_document(&body);
    }

    fn parse(&self, html: &scraper::Html) -> Vec<Item> {
        let mut items = Vec::new();

        let bind = self.selector();
        let its = html.select(&bind);

        for it in its.into_iter() {
            let title = it.inner_html();
            let url = it.value().attr("href").unwrap().to_string();
            items.push(Item { title, url });
        }

        items
    }
}

pub fn crawler_start() -> Result<HashMap<String, Vec<Item>>> {
    let mut map = HashMap::new();

    let hn = hn::HackerNews {
        url: "https://news.ycombinator.com/".to_string(),
        size: 10,
    };

    let hn_items = hn.crawl(&hn.url);

    map.insert(
        "hackernews".to_string(),
        hn_items.into_iter().take(hn.size).collect(),
    );

    let hupu = hupu::Hupu {
        url: "https://bbs.hupu.com/all-gambia".to_string(),
        size: 10,
        base_url: "https://bbs.hupu.com".to_string(),
    };

    let hupu_items = hupu.crawl(&hupu.url);

    let hupu_items = hupu_items
        .into_iter()
        .take(hupu.size)
        .map(|item| Item {
            url: format!("{}{}", hupu.base_url, item.url),
            ..item
        })
        .collect::<Vec<Item>>();

    map.insert("hupu".to_string(), hupu_items);

    let douban = douban::DouBan {
        url: "https://www.douban.com/group/explore".to_string(),
        size: 10,
    };

    let douban_items = douban.crawl(&douban.url);

    map.insert(
        "douban".to_string(),
        douban_items.into_iter().take(douban.size).collect(),
    );

    Ok(map)
}

pub fn output_to(path: String) {
    let map = crawler_start().unwrap();
    let json = serde_json::to_string_pretty(&map).unwrap();

    std::fs::write(path, json).unwrap();
}
