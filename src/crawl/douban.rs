use crate::crawl::Crawler;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

pub struct DouBan {
    pub url: String,
    pub size: usize,
}

impl Crawler for DouBan {
    fn selector(&self) -> scraper::Selector {
        scraper::Selector::parse("div.channel-item>div.bd>h3>a").unwrap()
    }

    fn request(&self, url: &str) -> scraper::Html {
        let mut hs = HeaderMap::new();
        hs.insert(
            USER_AGENT,
            HeaderValue::from_static(
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36",
            ),
        );

        let body = tokio::task::block_in_place(|| {
            let client = reqwest::blocking::Client::builder().build().unwrap();
            let body = client.get(url).headers(hs).send().unwrap().text().unwrap();

            body
        });

        return scraper::Html::parse_document(&body);
    }
}
