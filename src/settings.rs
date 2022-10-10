use std::fs;

use serde::Deserialize;

use crate::anime::Anime;
use crate::result::Result;
use crate::rss_feed::RssFeed;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub polling_interval: u64,
    pub aria_url: String,
    pub mongo_db_url: String,
    pub client_id: String,
    pub animes: Vec<Anime>,
    pub feeds: Vec<RssFeed>,
}

impl Settings {
    pub fn read() -> Result<Self> {
        let settings_yaml = fs::read_to_string("settings.yaml")?;
        Ok(serde_yaml::from_str(&settings_yaml)?)
    }
}
