use crate::crawl::Crawler;

pub struct HackerNews {
    pub url: String,
    pub size: usize,
}

impl Crawler for HackerNews {
    fn selector(&self) -> scraper::Selector {
        scraper::Selector::parse("span.titleline>a").unwrap()
    }
}
