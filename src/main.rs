use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use scraper;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;
use tokio;
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    title: String,
    url: String,
}

pub trait Crawler {
    fn crawl(&self, url: &str) -> Vec<Item> {
        let document = self.request(url);

        self.parse(&document)
    }

    fn selector(&self) -> scraper::Selector;

    fn request(&self, url: &str) -> scraper::Html {
        return tokio::runtime::Runtime::new().unwrap().block_on(async {
            let body = reqwest::get(url).await.unwrap().text().await.unwrap();

            return scraper::Html::parse_document(&body);
        });
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

pub struct HackerNews {
    url: String,
}

pub struct Hupu {
    url: String,
    size: usize,
    base_url: String,
}

pub struct DouBan {
    url: String,
}

impl Crawler for DouBan {
    fn selector(&self) -> scraper::Selector {
        scraper::Selector::parse("div.channel-item>div.bd>h3>a").unwrap()
    }

    fn request(&self, url: &str) -> scraper::Html {
        let mut hs = HeaderMap::new();
        hs.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36"));

        let client = reqwest::Client::new();

        return tokio::runtime::Runtime::new().unwrap().block_on(async {
            let body = client
                .get(url)
                .headers(hs)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            return scraper::Html::parse_document(&body);
        });
    }
}

impl Crawler for Hupu {
    fn selector(&self) -> scraper::Selector {
        scraper::Selector::parse("div.t-info>a").unwrap()
    }

    fn parse(&self, html: &scraper::Html) -> Vec<Item> {
        let mut items = Vec::new();

        let bind = self.selector();
        let its = html.select(&bind);

        for it in its.into_iter() {
            let title = it
                .select(&scraper::Selector::parse("span.t-title").unwrap())
                .next()
                .unwrap()
                .inner_html();
            let url = it.value().attr("href").unwrap().to_string();
            items.push(Item { title, url });
        }

        items
    }
}

impl Crawler for HackerNews {
    fn selector(&self) -> scraper::Selector {
        scraper::Selector::parse("span.titleline>a").unwrap()
    }
}

fn crawl_to_json() {
    let mut map = HashMap::new();

    let hn = HackerNews {
        url: "https://news.ycombinator.com/".to_string(),
    };

    let hn_items = hn.crawl(&hn.url);

    map.insert("hackernews".to_string(), hn_items);

    let hupu = Hupu {
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

    let douban = DouBan {
        url: "https://www.douban.com/group/explore".to_string(),
    };

    let douban_items = douban.crawl(&douban.url);

    map.insert("douban".to_string(), douban_items);

    // json with indent
    let json = serde_json::to_string_pretty(&map).unwrap();

    std::fs::write("mofish.json", json).unwrap();
}

fn main() {
    let ten_minutes = std::time::Duration::from_secs(60 * 10);

    if !std::path::Path::new("mofish.json").exists()
        || std::fs::metadata("mofish.json")
            .unwrap()
            .modified()
            .unwrap()
            .elapsed()
            .unwrap()
            > ten_minutes
    {
        crawl_to_json();
    }
}
