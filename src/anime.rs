use serde::Deserialize;

use crate::feed_source::source::SourceType;

#[derive(Debug, Deserialize)]
pub struct Anime {
    pub name: Option<String>,
    pub source: RssSource,
}

#[derive(Debug, Deserialize)]
pub struct RssSource {
    pub id: String,
    pub rss: SourceType,
}
