use anyhow::Result;
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlite::{Connection, State};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Link {
    pub id: i64,
    pub url: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
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
        "CREATE TABLE IF NOT EXISTS tops (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        url TEXT NOT NULL,
        site TEXT NOT NULL,
        title TEXT NOT NULL,
        idx INTEGER NOT NULL,
        created_at INTEGER NOT NULL
    );",
    )?;

    Ok(())
}

fn get_db_conn() -> Result<Connection> {
    let conn = sqlite::open("mofish.db")?;
    Ok(conn)
}

pub fn get_latest(sites: Vec<String>, expire: i64) -> Result<HashMap<String, Vec<Top>>> {
    let conn = get_db_conn()?;

    let mut m: HashMap<String, Vec<Top>> = HashMap::new();

    let sites_ph: Vec<String> = sites.iter().map(|_| "?".to_string()).collect();

    let mut stmt = conn.prepare(format!(
        "SELECT * FROM tops WHERE site IN ({}) AND created_at > :ctime ORDER BY idx DESC",
        sites_ph.join(",")
    ))?;

    for (i, site) in sites.iter().enumerate() {
        stmt.bind((i + 1, site.as_str()))?;
    }

    stmt.bind((":ctime", Local::now().timestamp() - expire))?;

    while let State::Row = stmt.next()? {
        let create_timestamp = stmt.read::<i64, _>("created_at")?;

        let top = Top {
            id: stmt.read::<i64, _>("id")?,
            url: stmt.read::<String, _>("url")?,
            site: stmt.read::<String, _>("site")?,
            title: stmt.read::<String, _>("title")?,
            index: stmt.read::<i64, _>("idx")?,
            created_at: NaiveDateTime::from_timestamp_opt(create_timestamp, 0)
                .unwrap()
                .to_string(),
        };

        let site = top.site.clone();
        if m.contains_key(&site) {
            m.get_mut(&site).unwrap().push(top);
        } else {
            m.insert(site, vec![top]);
        }
    }

    info!("get_latest result: {:?}", m);

    Ok(m)
}

fn escape(s: String) -> String {
    s.replace("'", "''")
}

pub fn insert_tops(tops: Vec<Top>) -> Result<()> {
    let conn = get_db_conn()?;

    for top in tops {
        let sql = format!(
            "INSERT INTO tops (url, site, title, idx, created_at) VALUES ('{}', '{}', '{}', {}, {})",
            escape(top.url),
            escape(top.site),
            escape(top.title),
            top.index,
            Local::now().timestamp(),
        );

        match conn.execute(sql.as_str()) {
            Err(e) => {
                error!("insert_tops error: {}", e);
                return Err(e.into());
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
        let sites = vec!["hackernews".to_string(), "hupu".to_string()];
        match get_latest(sites, None.unwrap_or(3600)) {
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
}
