use async_trait::async_trait;
use rss::Channel;

use crate::{result::Result, settings::Settings};

use super::source::{get_feed, Source, SourceType};

pub struct EraiRaws<'a> {
    settings: &'a Settings,
}

#[async_trait]
impl<'a> Source for EraiRaws<'a> {
    async fn get_feed(&self) -> Result<Channel> {
        let feed = get_feed(&self.settings.feeds, SourceType::EraiRaws)?;

        let content = reqwest::get(feed.url.to_owned()).await?.bytes().await?;
        let channel = Channel::read_from(&content[..])?;

        return Ok(channel);
    }
}

impl<'a> EraiRaws<'a> {
    pub fn new(settings: &'a Settings) -> Self {
        Self { settings }
    }
}
