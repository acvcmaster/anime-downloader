use serde::Deserialize;

use crate::feed_source::source::SourceType;

#[derive(Debug, Deserialize)]
pub struct RssFeed {
    #[serde(alias = "type")]
    pub feed_type: SourceType,
    pub pattern: String,
    pub url: String,
}
