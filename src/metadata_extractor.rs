use regex::Regex;

use crate::feed_source::source::{get_feed, SourceType};
use crate::metadata::Metadata;
use crate::result::Result;
use crate::settings::Settings;

pub struct MetadataExtractor {
    pattern: Regex,
}

impl MetadataExtractor {
    pub fn for_source(settings: &Settings, source_type: SourceType) -> Result<Self> {
        let feed = get_feed(&settings.feeds, source_type)?;

        Ok(Self {
            pattern: Regex::new(&feed.pattern)?,
        })
    }

    pub fn extract(&self, title: &str) -> Option<Metadata> {
        let captures = self.pattern.captures(title);

        if let Some(captures) = captures {
            let name = captures.name("name").map(|value| value.as_str());
            let episode = captures.name("episode").map(|value| value.as_str());

            return match (name, episode) {
                (Some(name), Some(episode)) => {
                    if let Ok(episode) = episode.to_owned().parse() {
                        return Some(Metadata::new(name.to_owned(), episode));
                    }

                    None
                }
                _ => None,
            };
        }

        None
    }
}
