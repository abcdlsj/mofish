use anyhow::Result;
use readability::extractor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadableLink {
    url: String,
    title: String,
    content: String,
}

pub async fn fetch_read(url: String) -> Result<ReadableLink> {
    let ret = extractor::scrape(&url)?;

    Ok(ReadableLink {
        url: url,
        title: ret.title,
        content: ret.content,
    })
}
