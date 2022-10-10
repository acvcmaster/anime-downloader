mod anime;
mod feed_source;
mod generic_error;
mod metadata;
mod metadata_extractor;
mod result;
mod rss_feed;
mod settings;

use std::sync::Arc;

use feed_source::source::{Source, SourceType};
use metadata_extractor::MetadataExtractor;
use settings::Settings;

use crate::result::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Arc::new(Settings::read()?);
    let source = SourceType::EraiRaws.get_source(&settings);
    let metadata_extrator = MetadataExtractor::for_source(&settings, SourceType::EraiRaws)?;

    let channel = source.get_feed().await?;

    for item in channel.items {
        let title = item.title.unwrap();
        let metadata = metadata_extrator.extract(&title);
        // println!("{:?}", item.title);
        if metadata.is_none() {
            println!("{title}");
        }

        // println!("{:?}", item.link);
        println!();
    }

    Ok(())
}
