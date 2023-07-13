use crate::crawl::{Crawler, Item};

pub struct Hupu {
    pub url: String,
    pub size: usize,
    pub base_url: String,
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
