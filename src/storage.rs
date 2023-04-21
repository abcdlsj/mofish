use anyhow::Result;
use chrono;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Link {
    pub id: i64,
    pub url: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Top {
    pub id: i64,
    pub url: String,
    pub site: String,
    pub title: String,
    pub index: i64,
    pub created_at: String,
}

pub fn init_storage() -> Result<()> {
    let conn = get_db_conn()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS links (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        url TEXT NOT NULL,
        title TEXT NOT NULL,
        content TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    );",
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tops (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        url TEXT NOT NULL,
        site TEXT NOT NULL,
        title TEXT NOT NULL,
        idx INTEGER NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    );",
    )?;

    Ok(())
}

fn get_db_conn() -> Result<sqlite::Connection> {
    let conn = sqlite::open("mofish.db")?;
    Ok(conn)
}

pub fn get_latest_top_links(
    sites: Vec<String>,
    _timestamp: Option<String>,
) -> Result<HashMap<String, Vec<Top>>> {
    let conn = sqlite::open("mofish.db")?;
    let mut m: HashMap<String, Vec<Top>> = HashMap::new();

    let mut stmt = conn.prepare(
        "SELECT * FROM tops WHERE site IN (:sites) AND created_at > datetime(:ctime, 'unixepoch') GROUP BY site ORDER BY idx DESC LIMIT 10",
    )?;
    stmt.bind((":sites", sites.join(",").as_str()))?;
    stmt.bind((":ctime", chrono::Utc::now().timestamp() - 3600))?;

    info!(":sites: {}", sites.join(",").as_str());
    info!(":ctime: {}", chrono::Utc::now().timestamp() - 3600);

    while let sqlite::State::Row = stmt.next()? {
        let top = Top {
            id: stmt.read::<i64, _>("id")?,
            url: stmt.read::<String, _>("url")?,
            site: stmt.read::<String, _>("site")?,
            title: stmt.read::<String, _>("title")?,
            index: stmt.read::<i64, _>("idx")?,
            created_at: stmt.read::<String, _>("create_at")?,
        };

        let site = top.site.clone();
        if m.contains_key(&site) {
            m.get_mut(&site).unwrap().push(top);
        } else {
            m.insert(site, vec![top]);
        }
    }

    Ok(m)
}

fn escape(s: String) -> String {
    s.replace("'", "''")
}

pub fn insert_tops(tops: Vec<Top>) -> Result<()> {
    let conn = get_db_conn()?;

    for top in tops {
        let sql = format!(
            "INSERT INTO tops (url, site, title, idx) VALUES ('{}', '{}', '{}', {})",
            escape(top.url),
            escape(top.site),
            escape(top.title),
            top.index
        );

        match conn.execute(sql.as_str()) {
            Err(e) => {
                error!("insert_tops error: {}", e);
            }
            _ => {}
        }
    }

    Ok(())
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_latest_top_links() {
        let sites = vec!["hackernews".to_string(), "lobsters".to_string()];
        match get_latest_top_links(sites, None) {
            Ok(m) => {
                println!("{:?}", m);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    #[test]
    fn test_init_storage() {
        match init_storage() {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    #[test]
    fn test_insert_tops() {
        let tops = vec![
            Top {
                url: "https://www.google.com".to_string(),
                site: "hackernews".to_string(),
                title: "google".to_string(),
                index: 1,
                id: 0,
                created_at: "".to_string(),
            },
            Top {
                url: "https://www.google.com".to_string(),
                site: "hackernews".to_string(),
                title: "google".to_string(),
                index: 2,
                id: 0,
                created_at: "".to_string(),
            },
            Top {
                url: "https://www.google.com".to_string(),
                site: "hackernews".to_string(),
                title: "google".to_string(),
                index: 3,
                id: 0,
                created_at: "".to_string(),
            },
        ];

        match insert_tops(tops) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
