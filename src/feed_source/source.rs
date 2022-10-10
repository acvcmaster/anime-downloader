use async_trait::async_trait;
use rss::Channel;
use serde::Deserialize;

use crate::{generic_error::GenericError, result::Result, rss_feed::RssFeed, settings::Settings};

use super::erairaws_source::EraiRaws;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum SourceType {
    EraiRaws,
}

#[async_trait]
pub trait Source {
    async fn get_feed(&self) -> Result<Channel>;
}

impl SourceType {
    pub fn get_source<'a>(&self, settings: &'a Settings) -> impl Source + 'a {
        match &self {
            SourceType::EraiRaws => EraiRaws::new(settings),
        }
    }
}

pub fn get_feed(rss_feeds: &[RssFeed], feed_type: SourceType) -> Result<&RssFeed> {
    match rss_feeds.iter().find(|feed| feed.feed_type == feed_type) {
        Some(url) => Ok(url),
        None => GenericError::from(format!("Feed not defined (type = '{:?}')", feed_type)),
    }
}
